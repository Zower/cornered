import 'package:cornered/gen/ffi.dart';
import 'package:flutter/material.dart';

class CommonPage extends StatelessWidget {
  const CommonPage(
      {Key? key,
      required this.title,
      required this.child,
      this.floatingActionButton,
      this.actions = const []})
      : super(key: key);

  final String title;
  final Widget child;
  final Widget? floatingActionButton;
  final List<Widget> actions;
  // final AppBar? appBar;

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: _appBar(),
      drawer: _drawer(),
      floatingActionButton: floatingActionButton,
      body: child,
    );
  }

  Drawer _drawer() {
    return Drawer(
      child: ListView(
        padding: EdgeInsets.zero,
        children: [
          const DrawerHeader(
            decoration: BoxDecoration(
              color: Colors.orange,
            ),
            child: Text('Cornered'),
          ),
          ListTile(
            title: const Text('Clear database'),
            onTap: () async {
              api.clearDb();
            },
          ),
        ],
      ),
    );
  }

  AppBar _appBar() {
    return AppBar(
      title: const Text('Cornered'),
      actions: actions,
    );
  }
}
