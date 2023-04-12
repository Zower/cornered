import 'package:flutter/material.dart';

class FutureHandledBuilder<T> extends StatefulWidget {
  const FutureHandledBuilder({
    Key? key,
    required this.future,
    required this.builder,
    WidgetBuilder? loadingBuilder,
    Widget Function(BuildContext context, Object? e)? errorBuilder,
  })  : loadingBuilder = loadingBuilder ?? _defaultLoadingBuilder,
        errorBuilder = errorBuilder ?? _defaultErrorBuilder,
        super(key: key);

  final Future<T> future;
  final Widget Function(BuildContext context, T t) builder;
  final WidgetBuilder loadingBuilder;
  final Widget Function(BuildContext context, Object? e) errorBuilder;

  @override
  State<FutureHandledBuilder<T>> createState() =>
      _FutureHandledBuilderState<T>();

  static Widget _defaultLoadingBuilder(context) =>
      const Center(child: CircularProgressIndicator());
  static Widget _defaultErrorBuilder(context, e) =>
      Text('Unexpected error: $e');
}

class _FutureHandledBuilderState<T> extends State<FutureHandledBuilder<T>> {
  @override
  Widget build(BuildContext context) {
    return FutureBuilder<T>(
      future: widget.future,
      builder: (context, AsyncSnapshot<T> snapshot) {
        if (snapshot.connectionState == ConnectionState.done) {
          if (snapshot.hasError) {
            return widget.errorBuilder(context, snapshot.error);
          } else {
            return widget.builder(context, snapshot.data as T);
          }
        } else {
          return widget.loadingBuilder(context);
        }
      },
    );
  }
}
