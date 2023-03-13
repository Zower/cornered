import 'package:cornered/common/common_page.dart';
import 'package:cornered/gen/ffi.dart';
import 'package:cornered/smooth_scroll.dart';
import 'package:file_picker/file_picker.dart';
import 'package:flutter/material.dart';
import 'package:flutter_html/flutter_html.dart';
import 'package:google_fonts/google_fonts.dart';

class Reader extends StatefulWidget {
  const Reader({Key? key, required this.path}) : super(key: key);

  final String path;

  @override
  State<Reader> createState() => _ReaderState();
}

class _ReaderState extends State<Reader> {
  String? _currentContent;
  FilePickerResult? _result;

  void _do() {
    api
        // path:
        //     "D:\\Downloads\\(The Wheel of Time 13) Jordan, Robert - Towers of Midnight (1).epub")
        .openDoc(path: widget.path)
        .then((_) => _setContent());
  }

  Future<void> _setContent() async {
    final content = await api.getContent();

    setState(() {
      _currentContent = content;
    });
  }

  @override
  void initState() {
    super.initState();

    _do();
  }

  @override
  Widget build(BuildContext context) {
    return CommonPage(
      title: 'Reader',
      child: _body(),
    );
  }

  Widget _body() {
    if (_currentContent == null) {
      return const Text('nah');
    }

    return GestureDetector(
      onHorizontalDragEnd: (r) {
        if (r.primaryVelocity! > 100) {
          api.goPrev().then((value) => _setContent());
        } else if (r.primaryVelocity! < -110) {
          api.goNext().then((_) => _setContent());
        }
      },
      child: Container(
        height: double.infinity,
        width: double.infinity,
        color: const Color(0xfffbf0d9),
        child: SmoothScroll(
          // controller: controller,
          // physics: physics,
          // cacheExtent: double.maxFinite,
          children: [
            DefaultTextStyle(
              style: const TextStyle(
                fontSize: 22,
                height: 1.5,
              ),
              child: SelectionArea(
                onSelectionChanged: (selection) {
                  debugPrint(selection?.plainText.toString());
                },
                child: Html(
                  data: _currentContent!,
                  style: {
                    "p": Style.fromTextStyle(
                      GoogleFonts.ebGaramond().copyWith(
                        fontSize: 20,
                        fontWeight: FontWeight.w300,
                      ),
                    ).merge(
                      Style(
                        textAlign: TextAlign.justify,
                      ),
                    ),
                    "html": Style(
                      padding: const EdgeInsets.symmetric(
                        horizontal: 32,
                      ),
                    ),
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
            Container(
              margin: const EdgeInsets.only(top: 128, bottom: 32, left: 64, right: 64),
              height: 2,
              width: double.infinity,
              color: Colors.black,
            ),
            TextButton(
              onPressed: () async {
                final userCode = await api.auth();

                if (!mounted) return;

                final fut = api.poll();

                await showDialog<void>(
                  context: context,
                  barrierDismissible: false,
                  builder: (BuildContext context) {
                    fut.then((_) => Navigator.of(context).pop());

                    return AlertDialog(
                      title: const Text('Code'),
                      content: SingleChildScrollView(
                        child: ListBody(
                          children: [
                            SelectableText('userCode: $userCode'),
                          ],
                        ),
                      ),
                    );
                  },
                );
              },
              child: Text('sync'),
            ),
            TextButton(
              onPressed: () async {
                // api.goPrev().then((_) => _setContent());
                // await api.sync2(path: "D:\\Downloads\\(The Wheel of Time 13) Jordan, Robert - Towers of Midnight (1).epub");
                await api.sync2(path: _result!.files.single.path!);
                debugPrint('sync2 done');
              },
              child: Text('sync 2'),
            ),
          ],
        ),
      ),
    );
  }
}
