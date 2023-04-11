import 'package:cornered/common/theme.dart';
import 'package:cornered/gen/ffi.dart';
import 'package:cornered/views/library.dart';
import 'package:flutter/foundation.dart';
import 'package:flutter/material.dart';
import 'package:path_provider/path_provider.dart';

void main() async {
  WidgetsFlutterBinding.ensureInitialized();

  try {
    await booksApi.initApp(
        dataDir: (await getApplicationSupportDirectory()).path);
  } catch (e) {
    if (kReleaseMode) {
      rethrow;
    }
  }

  runApp(const Cornered());
}

class Cornered extends StatelessWidget {
  const Cornered({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'Cornered',
      debugShowCheckedModeBanner: false,
      theme: brownOrange,
      darkTheme: brownOrangeDark,
      home: const Library(),
    );
  }
}
