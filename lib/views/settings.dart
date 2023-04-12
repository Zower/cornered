import 'package:cornered/common/common_page.dart';
import 'package:cornered/views/synchronize.dart';
import 'package:flutter/material.dart';
import 'package:page_transition/page_transition.dart';

class Settings extends StatefulWidget {
  const Settings({Key? key, this.dummy = false}) : super(key: key);

  @override
  State<Settings> createState() => _SettingsState();

  final bool dummy;
}

class _SettingsState extends State<Settings> {
  @override
  Widget build(BuildContext context) {
    final x = 3;

    return CommonPage(
      title: "Settings $x 13 test",
      showDrawer: false,
      child: ListView(
        children: [
          ListTile(
            leading: const Icon(Icons.sync),
            title: const Text("Synchronization"),
            trailing: const Icon(Icons.chevron_right),
            onTap: !widget.dummy
                ? () =>
                    _pushSetting(context, const SynchronizationSettingPage())
                : null,
          ),
        ],
      ),
    );
  }

  Future<void> _pushSetting(BuildContext context, Widget child) async {
    await Navigator.push(
      context,
      PageTransition(
        childCurrent: const Settings(dummy: true),
        type: PageTransitionType.rightToLeftJoined,
        duration: const Duration(milliseconds: 300),
        reverseDuration: const Duration(milliseconds: 250),
        curve: Curves.easeOutCubic,
        child: child,
      ),
    );
  }

  Widget _settingPage(String title, Widget child,
      {Widget? floatingActionButton}) {
    return CommonPage(
        title: title,
        showDrawer: false,
        floatingActionButton: floatingActionButton,
        child: child);
  }
}
