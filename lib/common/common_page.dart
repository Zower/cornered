import 'package:cornered/gen/ffi.dart';
import 'package:flutter/material.dart';
import 'package:path_provider/path_provider.dart';

class CommonPage extends StatelessWidget {
  const CommonPage(
      {Key? key,
      required this.title,
      required this.child,
      this.floatingActionButton})
      : super(key: key);

  final String title;
  final Widget child;
  final Widget? floatingActionButton;
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
              api.clearDb(
                  path: (await getApplicationDocumentsDirectory()).path);
            },
          ),
        ],
      ),
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
