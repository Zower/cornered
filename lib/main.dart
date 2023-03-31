import 'package:cornered/views/library.dart';
import 'package:flutter/material.dart';

void main() {
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

class Cornered extends StatelessWidget {
  const Cornered({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return const Library();
  }
}
