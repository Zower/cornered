import 'package:flutter/material.dart';

T? runCatching<T>(T Function() f) {
  try {
    return f();
  } catch (e) {
    debugPrint('An unexpected error occurred: ${e.toString()}');
    // showDialog(
    //   context: context(),
    //   builder: (context) => AlertDialog(
    //     title: const Text("Crash"),
    //     content: Text('An unexpected error occurred: ${e.toString()}'),
    //     actions: [
    //       TextButton(
    //         onPressed: () => Navigator.pop(context),
    //         child: const Text("Close"),
    //       ),
    //     ],
    //   ),
    // );

    return null;
  }
}
