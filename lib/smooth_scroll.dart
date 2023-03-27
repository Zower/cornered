import 'dart:io';
import 'dart:math';

import 'package:flutter/gestures.dart';
import 'package:flutter/material.dart';

class SmoothScroll extends StatefulWidget {
  const SmoothScroll(
      {super.key,
      required this.children,
      this.onScrollEnd,
      required this.initialOffset});

  final List<Widget> children;
  final double initialOffset;
  final void Function(double offset)? onScrollEnd;

  @override
  State<SmoothScroll> createState() => _SmoothScrollState();
}

class _SmoothScrollState extends State<SmoothScroll> {
  late final ScrollController controller;

  double? _desiredOffset = 0;

  @override
  void initState() {
    super.initState();

    controller = ScrollController(initialScrollOffset: widget.initialOffset);
  }

  void _onPointerSignal(PointerSignalEvent details) {
    if (details is PointerScrollEvent) {
      _desiredOffset ??= controller.position.pixels;

      _desiredOffset = max(
        controller.position.minScrollExtent,
        min(
          controller.position.maxScrollExtent,
          _desiredOffset! + (details.scrollDelta.dy * 1.5),
        ),
      );
      controller.animateTo(
        _desiredOffset!,
        duration: const Duration(milliseconds: 230),
        curve: Curves.easeOutSine,
      );
    }
    // controller.animateTo(
    //   controller.position.pixels + (details.scrollDelta.dy * 10),
    //   duration: Duration(milliseconds: 200),
    //   curve: Curves.easeInOut,
    // );
  }

  @override
  Widget build(BuildContext context) {
    return NotificationListener<ScrollNotification>(
      onNotification: (notification) {
        if (notification is ScrollEndNotification) {
          widget.onScrollEnd?.call(controller.offset);
        }

        return false;
      },
      child: _body(context),
    );
  }

  Widget _body(BuildContext context) {
    if (Platform.isLinux || Platform.isWindows || Platform.isMacOS) {
      return Listener(
        onPointerSignal: _onPointerSignal,
        child: ListView(
          physics: const NeverScrollableScrollPhysics(),
          controller: controller,
          cacheExtent: double.maxFinite,
          children: widget.children,
        ),
      );
    }

    return ListView(
      controller: controller,
      children: widget.children,
    );
  }
}
