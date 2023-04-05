import 'package:cornered/gen/ffi.dart';
import 'package:cornered/views/library.dart';
import 'package:flutter/foundation.dart';
import 'package:flutter/material.dart';
import 'package:path_provider/path_provider.dart';

void main() async {
  WidgetsFlutterBinding.ensureInitialized();

  try {
    await booksApi.initApp(dataDir: (await getApplicationSupportDirectory()).path);
  } catch (e) {
    if (kReleaseMode) {
      rethrow;
    }
  }

  runApp(const MyApp());
}

class MyApp extends StatelessWidget {
  const MyApp({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'Cornered',
      theme: ThemeData(
        primarySwatch: Colors.orange,
      ),
      debugShowCheckedModeBanner: false,
      home: const Cornered(),
    );
  }
}

class MyCustom extends MaterialScrollBehavior {}

class Cornered extends StatelessWidget {
  const Cornered({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return const Library();
  }
}
