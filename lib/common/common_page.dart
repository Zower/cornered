import 'package:cornered/gen/ffi.dart';
import 'package:cornered/views/library.dart';
import 'package:flutter/material.dart';
import 'package:page_transition/page_transition.dart';

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
      drawer: _drawer(context),
      floatingActionButton: floatingActionButton,
      body: child,
    );
  }

  Drawer _drawer(BuildContext context) {
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
            leading: const Icon(Icons.library_books),
            title: const Text('Library'),
            onTap: () async {
              Navigator.pushAndRemoveUntil(
                context,
                PageTransition(
                    child: const Library(), type: PageTransitionType.fade),
                ModalRoute.withName('/'),
              );
            },
          ),
          ListTile(
            leading: const Icon(Icons.clear),
            title: const Text('Clear database'),
            onTap: () async {
              booksApi.clearDb();
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
