import 'package:flutter/material.dart';
import 'package:flutter_html/flutter_html.dart';
import 'package:dyn_mouse_scroll/dyn_mouse_scroll.dart';
import 'package:flutter_rust_bridge_template/smooth_scroll.dart';
import 'package:google_fonts/google_fonts.dart';
import 'ffi.dart' if (dart.library.html) 'ffi_web.dart';

void main() {
  runApp(const MyApp());
}

class MyApp extends StatelessWidget {
  const MyApp({Key? key}) : super(key: key);

  // This widget is the root of your application.
  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'Flutter Demo',
      theme: ThemeData(
        primarySwatch: Colors.blue,
      ),
      home: const MyHomePage(title: 'Flutter Demo Home Page'),
    );
  }
}

class MyHomePage extends StatefulWidget {
  const MyHomePage({Key? key, required this.title}) : super(key: key);

  // This widget is the home page of your application. It is stateful, meaning
  // that it has a State object (defined below) that contains fields that affect
  // how it looks.

  // This class is the configuration for the state. It holds the values (in this
  // case the title) provided by the parent (in this case the App widget) and
  // used by the build method of the State. Fields in a Widget subclass are
  // always marked "final".

  final String title;

  @override
  State<MyHomePage> createState() => _MyHomePageState();
}

class _MyHomePageState extends State<MyHomePage> {
  String? _currentContent;

  @override
  void initState() {
    super.initState();

    api
        .openDoc(
            path:
                "D:\\Downloads\\(The Wheel of Time 13) Jordan, Robert - Towers of Midnight (1).epub")
        .then((_) => _setContent());
  }

  Future<void> _setContent() async {
    final content = await api.getContent();

    setState(() {
      _currentContent = content;
    });
  }

  @override
  Widget build(BuildContext context) {
    if (_currentContent == null) {
      return const Text('nah');
    }

    return Scaffold(
      body: SmoothScroll(
        // controller: controller,
        // physics: physics,
        // cacheExtent: double.maxFinite,
        children: [
          TextButton(
            onPressed: () {
              api.goNext().then((_) => _setContent());
            },
            child: Text('go next'),
          ),
          Container(
            color: const Color(0xfffbf0d9),
            child: DefaultTextStyle(
              style: const TextStyle(
                fontSize: 22,
                height: 1.5,
              ),
              child: Html(
                data: _currentContent!,
                style: {
                  "p": Style.fromTextStyle(GoogleFonts.ebGaramond()
                      .copyWith(fontSize: 20, fontWeight: FontWeight.w300)),
                  // "p": Style(
                  //   // margin: EdgeInsets.symmetric(),
                  //   fontSize: FontSize(22),
                  //   fontFamily:
                  //   // display: Display.BLOCK,
                  // ),
                },
                customRender: {
                  "svg": (context, element) {
                    debugPrint(context.toString());
                    debugPrint(element.toString());

                    return Container(height: 30, width: 30, color: Colors.red);
                  }
                },
                customImageRenders: {
                  (context, element) {
                    debugPrint(context.toString());
                    debugPrint(element.toString());

                    return true;
                  }: (context, ctx, element) => Container()
                },
              ),
            ),
          ),
        ],
      ),
    );
  }
}
