import 'package:cornered/views/library.dart';
import 'package:cornered/views/settings.dart';
import 'package:flutter/material.dart';
import 'package:page_transition/page_transition.dart';

class CommonPage extends StatelessWidget {
  const CommonPage(
      {Key? key,
      required this.title,
      required this.child,
      this.appBar,
      this.floatingActionButton,
      this.drawer,
      this.showDrawer = true,
      this.showAppBar = true,
      this.actions = const []})
      : super(key: key);

  final String title;
  final Widget child;
  final Widget? floatingActionButton;
  final List<Widget> actions;
  final AppBar? appBar;
  final Widget? drawer;
  final bool showDrawer;
  final bool showAppBar;

  @override
  Widget build(BuildContext context) {
    return SafeArea(
      child: Scaffold(
        appBar: showAppBar ? appBar ?? _appBar() : null,
        drawer: showDrawer ? drawer ?? _drawer(context) : null,
        floatingActionButton: floatingActionButton,
        body: child,
      ),
    );
  }

  Drawer _drawer(BuildContext context) {
    return Drawer(
      child: ListView(
        padding: EdgeInsets.zero,
        children: [
          DrawerHeader(
            decoration: BoxDecoration(color: Theme.of(context).primaryColor),
            child: Text(title),
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
            leading: const Icon(Icons.settings),
            title: const Text('Settings'),
            onTap: () async {
              Navigator.push(
                context,
                PageTransition(
                    child: const Settings(), type: PageTransitionType.fade),
              );
            },
          ),
        ],
      ),
    );
  }

  AppBar _appBar() {
    return AppBar(
      title: Text(title),
      actions: actions,
    );
  }
}
