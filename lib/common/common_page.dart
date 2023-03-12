import 'package:flutter/material.dart';

class CommonPage extends StatelessWidget {
  const CommonPage({Key? key, required this.child}) : super(key: key);

  final Widget child;
  // final AppBar? appBar;

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: _appBar(),
      body: child,
    );
  }

  AppBar _appBar() {
    return AppBar(
      title: const Text('Cornered'),
      actions: [
        IconButton(
          icon: const Icon(Icons.settings),
          onPressed: () {},
        ),
      ],
    );
  }
}
