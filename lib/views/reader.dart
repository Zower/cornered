import 'dart:math';
import 'dart:typed_data';

import 'package:collection/collection.dart';
import 'package:cornered/common/common_page.dart';
import 'package:cornered/common/future_handled_builder.dart';
import 'package:cornered/gen/bridge_generated_1.dart';
import 'package:cornered/gen/ffi.dart';
import 'package:cornered/smooth_scroll.dart';
import 'package:flutter/material.dart';
import 'package:flutter_html/flutter_html.dart';
import 'package:google_fonts/google_fonts.dart';

class Reader extends StatefulWidget {
  const Reader({Key? key, required this.book, required this.db})
      : super(key: key);

  final Book book;
  final Database db;

  @override
  State<Reader> createState() => _ReaderState();
}

class _ReaderState extends State<Reader> {
  DocumentId? _id;
  String? _currentContent;
  int _chapter = 0;
  final ValueNotifier<String> _currentSelection = ValueNotifier('');

  double _fontSize = 18;

  double _offset = 0;

  @override
  void initState() {
    super.initState();

    _chapter = widget.book.position.chapter;

    _init();
  }

  void _init() async {
    final id =
        await api.openDoc(path: widget.book.path, initialChapter: _chapter);

    setState(() {
      _id = id;
    });

    await _setContent();
  }

  Future<void> _setContent() async {
    final content = await api.getContent(id: _id!);

    setState(() {
      // TODO: type of text
      _currentContent = content.content;
    });
  }

  @override
  Widget build(BuildContext context) {
    return WillPopScope(
      onWillPop: () {
        Navigator.pop(
            context,
            Book(
              uuid: widget.book.uuid,
              path: widget.book.path,
              position: Position(
                chapter: _chapter,
                offset: _offset,
              ),
            ));
        return Future.value(false);
      },
      child: CommonPage(
        title: 'Reader',
        actions: [
          PopupMenuButton(
            itemBuilder: (context) => <PopupMenuEntry<dynamic>>[
              PopupMenuWidget(
                child: StatefulBuilder(builder: (context, setInnerState) {
                  return Padding(
                    padding: const EdgeInsets.symmetric(horizontal: 2.0),
                    child: Card(
                      elevation: 4,
                      color: Colors.grey.shade400,
                      child: ListTile(
                        contentPadding: const EdgeInsets.symmetric(
                            vertical: 8, horizontal: 16),
                        title: Text('Font size: ${_fontSize.roundToDouble()}'),
                        subtitle: Slider(
                          value: _fontSize.roundToDouble(),
                          max: 35,
                          onChanged: (val) {
                            setInnerState(() {
                              _fontSize = val;
                            });
                          },
                          onChangeEnd: (val) {
                            setState(() {
                              setInnerState(() {
                                _fontSize = val;
                              });
                            });
                          },
                        ),
                      ),
                    ),
                  );
                }),
              ),
            ],
          )
        ],
        child: _body(),
      ),
    );
  }

  Widget _body() {
    if (_currentContent == null) {
      return Container(
        color: const Color(0xfffbf0d9),
      );
    }

    return GestureDetector(
      onHorizontalDragEnd: (r) async {
        if (r.primaryVelocity! > 100) {
          final value = await api.goPrev(id: _id!);

          setState(() {
            // TODO: type of text
            _currentContent = value.content;
            _chapter--;
          });

          await widget.db.updateProgress(
              id: widget.book.uuid, chapter: _chapter, offset: 0);
        } else if (r.primaryVelocity! < -110) {
          final value = await api.goNext(id: _id!);

          setState(() {
            // TODO: type of text
            _currentContent = value.content;
            _chapter++;
          });

          await widget.db.updateProgress(
              id: widget.book.uuid, chapter: _chapter, offset: 0);
        }
      },
      child: SelectionArea(
        onSelectionChanged: (selection) {
          final newSelection = selection?.plainText ?? '';

          if (_currentSelection.value.contains(newSelection) &&
              !_currentSelection.value.contains(' ')) {
            return;
          }

          _currentSelection.value = selection?.plainText ?? '';
        },
        selectionControls:
            DictionarySelectionControls(selection: _currentSelection),
        child: Container(
          height: double.infinity,
          width: double.infinity,
          color: const Color(0xfffbf0d9),
          child: SmoothScroll(
            initialOffset: widget.book.position.offset,
            onScrollEnd: (offset) async {
              await widget.db.updateProgress(
                id: widget.book.uuid,
                chapter: _chapter,
                offset: offset,
              );

              setState(() {
                _offset = offset;
              });
            },
            children: [
              DefaultTextStyle(
                style: TextStyle(
                  fontSize: _fontSize,
                  height: 1.5,
                ),
                child: Html(
                  data: _currentContent!,
                  style: {
                    "p": Style.fromTextStyle(
                      GoogleFonts.ebGaramond().copyWith(
                        fontSize: _fontSize,
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
                  },
                  customRender: {
                    "svg": (context, element) {
                      debugPrint(context.toString());
                      debugPrint(element.toString());

                      return Container(
                          height: 30, width: 30, color: Colors.red);
                    }
                  },
                  customImageRenders: {
                    (context, element) {
                      debugPrint(context.toString());
                      debugPrint(element.toString());

                      return true;
                    }: (context, ctx, element) => FutureBuilder(
                          future: api.getResource(
                            id: _id!,
                            path: element!.attributes['src']!,
                          ),
                          builder: (ctx, AsyncSnapshot<Uint8List> snapshot) {
                            if (snapshot.hasData) {
                              return Image.memory(snapshot.data!);
                            }

                            return const SizedBox.shrink();
                          },
                        ),
                  },
                ),
              ),
              // Container(
              //   margin: const EdgeInsets.only(top: 128, bottom: 32, left: 64, right: 64),
              //   height: 2,
              //   width: double.infinity,
              //   color: Colors.black,
              // ),
              // TextButton(
              //   onPressed: () async {
              //     final userCode = await api.auth();
              //
              //     if (!mounted) return;
              //
              //     final fut = api.poll();
              //
              //     await showDialog<void>(
              //       context: context,
              //       barrierDismissible: false,
              //       builder: (BuildContext context) {
              //         fut.then((_) => Navigator.of(context).pop());
              //
              //         return AlertDialog(
              //           title: const Text('Code'),
              //           content: SingleChildScrollView(
              //             child: ListBody(
              //               children: [
              //                 SelectableText('userCode: $userCode'),
              //               ],
              //             ),
              //           ),
              //         );
              //       },
              //     );
              //   },
              //   child: const Text('sync'),
              // ),
              // TextButton(
              //   onPressed: () async {
              //     await api.sync2(path: _result!.files.single.path!);
              //     debugPrint('sync2 done');
              //   },
              //   child: const Text('sync 2'),
              // ),
            ],
          ),
        ),
      ),
    );
  }
}

class DictionarySelectionControls extends MaterialTextSelectionControls {
  final ValueNotifier<String> selection;

  DictionarySelectionControls({required this.selection});

  @override
  Widget buildHandle(
      BuildContext context, TextSelectionHandleType type, double textHeight,
      [VoidCallback? onTap]) {
    switch (Theme.of(context).platform) {
      case TargetPlatform.iOS:
      case TargetPlatform.android:
        return super.buildHandle(context, type, textHeight);
      case TargetPlatform.fuchsia:
      case TargetPlatform.macOS:
      case TargetPlatform.linux:
      case TargetPlatform.windows:
        return const SizedBox.shrink();
    }
  }

  @override
  Widget buildToolbar(
    BuildContext context,
    Rect globalEditableRegion,
    double textLineHeight,
    Offset selectionMidpoint,
    List<TextSelectionPoint> endpoints,
    TextSelectionDelegate delegate,
    ClipboardStatusNotifier? clipboardStatus,
    Offset? lastSecondaryTapDownPosition,
  ) {
    return LayoutBuilder(builder: (context, constraints) {
      double width = constraints.maxWidth;
      double height = constraints.maxHeight - endpoints.first.point.dy - 80;

      double x = 0.0;
      double y = endpoints.first.point.dy + 80;

      switch (Theme.of(context).platform) {
        case TargetPlatform.iOS:
        case TargetPlatform.android:
          break;
        case TargetPlatform.fuchsia:
        case TargetPlatform.macOS:
        case TargetPlatform.linux:
        case TargetPlatform.windows:
          width = min(constraints.maxWidth, 500);
          x = endpoints.first.point.dx;

          if (endpoints.first.point.dx + width > constraints.maxWidth) {
            x = endpoints.first.point.dx - width;
          }

          break;
      }

      if (endpoints.first.point.dy > constraints.maxHeight / 2) {
        height = min(constraints.maxHeight / 2, 250);
        y = endpoints.first.point.dy - height;
      }

      return Transform.translate(
        offset: Offset(x, y),
        child: Align(
          alignment: Alignment.topLeft,
          child: LayoutBuilder(builder: (context, constraints) {
            return ConstrainedBox(
              // width: width,
              // height: height,
              constraints: BoxConstraints(
                minWidth: width,
                minHeight: 200,
                maxWidth: width,
                maxHeight: height,
              ),
              child: Material(
                type: MaterialType.transparency,
                child: Container(
                  padding: const EdgeInsets.all(8),
                  margin: const EdgeInsets.all(8),
                  decoration: BoxDecoration(
                    borderRadius: BorderRadius.circular(10),
                    color: Colors.white,
                    boxShadow: [
                      BoxShadow(
                        color: Colors.black.withOpacity(0.25),
                        spreadRadius: 0,
                        blurRadius: 4,
                        offset:
                            const Offset(0, 4), // changes position of shadow
                      ),
                    ],
                  ),
                  child: FutureHandledBuilder(
                    future: api.getDefinition(word: selection.value),
                    loadingBuilder: (context) => const SizedBox(
                        height: 100,
                        child:
                            const Center(child: CircularProgressIndicator())),
                    errorBuilder: (context, error) =>
                        const Text('Unable to fetch word.'),
                    builder: (context, Definitions snapshot) {
                      final meanings = snapshot.meanings
                          .map(
                            (e) => Column(
                              children: [
                                ...e.definitions
                                    .map((e) {
                                      return [
                                        Text(e.definition),
                                        if (e.example != null) ...[
                                          const SizedBox(height: 8),
                                          Text('Example: ${e.example}'),
                                        ]
                                      ];
                                    })
                                    .toList()
                                    .flattened
                              ],
                            ),
                          )
                          .toList();

                      return ListView.separated(
                        itemBuilder: (ctx, i) => meanings[i],
                        shrinkWrap: true,
                        separatorBuilder: (ctx, i) => Container(
                          height: 1,
                          decoration: BoxDecoration(
                            borderRadius: BorderRadius.circular(99),
                            color: Colors.black,
                          ),
                          margin: const EdgeInsets.symmetric(vertical: 8),
                        ),
                        itemCount: meanings.length,
                      );
                    },
                  ),
                ),
              ),
            );
          }),
        ),
      );
    });
  }
}

class PopupMenuWidget extends PopupMenuEntry<Never> {
  const PopupMenuWidget({super.key, required this.child});

  final Widget child;

  @override
  // TODO: implement height
  double get height => throw UnimplementedError();

  @override
  bool represents(void value) {
    // TODO: implement represents
    throw UnimplementedError();
  }

  @override
  State<PopupMenuWidget> createState() => _PopupMenuWidgetState();
}

class _PopupMenuWidgetState extends State<PopupMenuWidget> {
  @override
  Widget build(BuildContext context) {
    return widget.child;
  }
}
