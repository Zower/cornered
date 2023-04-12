import 'package:cornered/common/common_page.dart';
import 'package:cornered/common/future_handled_builder.dart';
import 'package:cornered/gen/ffi.dart';
import 'package:cornered/gen/util_generated.dart';
import 'package:cornered/utils.dart';
import 'package:flutter/gestures.dart';
import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:url_launcher/url_launcher.dart';

class SynchronizationSettingPage extends StatefulWidget {
  const SynchronizationSettingPage({Key? key}) : super(key: key);

  @override
  State<SynchronizationSettingPage> createState() => _SynchronizationSettingPageState();
}

class _SynchronizationSettingPageState extends State<SynchronizationSettingPage> {
  @override
  Widget build(BuildContext context) {
    final child = FutureHandledBuilder(
      future: utilsApi.getUsers(),
      builder: (context, List<GithubUser> users) {
        if (users.isEmpty) {
          return const Center(child: Text("No users"));
        }

        return ListView.builder(
          itemCount: users.length,
          itemBuilder: (context, index) {
            final user = users[index];

            return ListTile(
              leading: const Icon(Icons.account_box_outlined),
              title: Text(user.displayName),
              trailing: user.isPrimary ? const Icon(Icons.check) : null,
            );
          },
        );
      },
    );

    return CommonPage(
      title: "Sync",
      showDrawer: false,
      floatingActionButton: FloatingActionButton(
        onPressed: () async {
          final response = await runCatching(() => utilsApi.auth());

          if (response == null) {
            _errorDialog('Failed to authenticate');
          }

          if (!mounted) return;

          try {
            final fut = utilsApi.poll(ongoing: response!);

            await showDialog<void>(
              context: context,
              barrierDismissible: false,
              builder: (BuildContext context) {
                fut.then((_) {
                  debugPrint('aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa');
                  Navigator.of(context).pop();
                });

                return WillPopScope(
                  onWillPop: () async => false,
                  child: AlertDialog(
                    title: const Text('Authenticate'),
                    actions: [
                      TextButton(
                        onPressed: () => Navigator.of(context).pop(),
                        child: const Text("Cancel"),
                      ),
                    ],
                    content: SingleChildScrollView(
                      child: ListBody(
                        children: [
                          SelectableText.rich(
                            TextSpan(
                              children: [
                                const TextSpan(text: "Tap to copy the code: "),
                                TextSpan(
                                  text: response.userCode,
                                  style: Theme.of(context).textTheme.headlineMedium!.copyWith(backgroundColor: Theme.of(context).colorScheme.background),
                                  recognizer: TapGestureRecognizer()
                                    ..onTap = () {
                                      Clipboard.setData(ClipboardData(text: response.userCode));
                                    },
                                ),
                              ],
                            ),
                          ),
                          const SizedBox(height: 16),
                          SelectableText.rich(
                            TextSpan(
                              children: [
                                const TextSpan(text: 'Then navigate to: '),
                                TextSpan(
                                  text: response.verificationUri,
                                  style: Theme.of(context).textTheme.bodyMedium!.copyWith(color: Theme.of(context).colorScheme.primary, decoration: TextDecoration.underline),
                                  recognizer: TapGestureRecognizer()
                                    ..onTap = () {
                                      launchUrl(
                                        Uri.parse(
                                          response.verificationUri,
                                        ),
                                        mode: LaunchMode.externalApplication,
                                      );
                                    },
                                ),
                              ],
                            ),
                          ),
                        ],
                      ),
                    ),
                  ),
                );
              },
            );
          } catch (e) {
            _errorDialog('Failed to authenticate: $e');
          }

          setState(() {});
        },
        child: const Icon(Icons.add),
      ),
      child: child,
    );
  }

  void _errorDialog(String message) {
    showDialog<void>(
      context: context,
      barrierDismissible: false,
      builder: (BuildContext context) {
        return AlertDialog(
          title: const Text('Error'),
          content: SingleChildScrollView(
            child: ListBody(
              children: [
                Text(message),
              ],
            ),
          ),
          actions: [
            TextButton(
              onPressed: () => Navigator.of(context).pop(),
              child: const Text("Ok"),
            ),
          ],
        );
      },
    );
  }
}
