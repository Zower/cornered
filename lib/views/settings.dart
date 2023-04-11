import 'package:cornered/common/common_page.dart';
import 'package:flutter/material.dart';
import 'package:page_transition/page_transition.dart';

class Settings extends StatelessWidget {
  const Settings({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return CommonPage(
      title: "Settings",
      showDrawer: false,
      child: ListView(
        children: [
          ListTile(
            leading: const Icon(Icons.sync),
            title: const Text("Synchronization"),
            trailing: const Icon(Icons.chevron_right),
            onTap: () {
              Navigator.push(
                context,
                PageTransition(
                  childCurrent: const Settings(),
                  type: PageTransitionType.rightToLeftJoined,
                  duration: const Duration(milliseconds: 300),
                  reverseDuration: const Duration(milliseconds: 300),
                  curve: Curves.easeOutCubic,
                  child: _sync(),
                ),
              );
            },
          ),
        ],
      ),
    );
  }

  Widget _sync() {
    return _settingPage(
      "Sync",
      const Text("Sync"),
    );
  }

  Widget _settingPage(String title, Widget child) {
    return CommonPage(title: title, showDrawer: false, child: child);
  }
}
