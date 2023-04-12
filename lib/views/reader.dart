import 'dart:math';
import 'dart:typed_data';
import 'dart:ui' as ui;

import 'package:collection/collection.dart';
import 'package:cornered/common/common_page.dart';
import 'package:cornered/common/future_handled_builder.dart';
import 'package:cornered/gen/ffi.dart';
import 'package:cornered/gen/util_generated.dart';
import 'package:cornered/smooth_scroll.dart';
import 'package:cornered/utils.dart';
import 'package:flutter/material.dart';
import 'package:flutter_html/flutter_html.dart' as html;
import 'package:google_fonts/google_fonts.dart';
import 'package:html/dom.dart' as dom;

class Reader extends StatefulWidget {
  const Reader({Key? key, required this.book, required this.db})
      : super(key: key);

  final Book book;
  final Database db;

  @override
  State<Reader> createState() => _ReaderState();
}

class _ReaderState extends State<Reader> {
  OpenDocument? _document;
  dom.Document? _htmlDocument;
  int _chapter = 0;
  final ValueNotifier<String> _currentSelection = ValueNotifier('');

  final Map<String, Uint8List> _images = {};

  TextStyle Function({
    TextStyle? textStyle,
    Color? color,
    Color? backgroundColor,
    double? fontSize,
    FontWeight? fontWeight,
    FontStyle? fontStyle,
    double? letterSpacing,
    double? wordSpacing,
    TextBaseline? textBaseline,
    double? height,
    Locale? locale,
    Paint? foreground,
    Paint? background,
    List<ui.Shadow>? shadows,
    List<ui.FontFeature>? fontFeatures,
    TextDecoration? decoration,
    Color? decorationColor,
    TextDecorationStyle? decorationStyle,
    double? decorationThickness,
  }) _textStyle = GoogleFonts.ebGaramond;

  double _fontSize = 18;
  double _paddingPercent = 32;
  // bool _showIcons = true;

  double _offset = 0;
  final ValueNotifier<double> _offsetNotifier = ValueNotifier(0);

  @override
  void initState() {
    super.initState();

    _chapter = widget.book.position.chapter;

    _init();
  }

  void _init() async {
    await runCatching(() async {
      final document = await booksApi.openDoc(
        path: widget.book.path,
        initialChapter: _chapter,
      );

      setState(() {
        _document = document;
      });
    });

    await _getAndSetContent();
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
        actions: _actions(),
        child: _body(),
      ),
    );
  }

  Widget _body() {
    if (_htmlDocument == null) {
      return Container();
    }

    return Stack(
      children: [
        GestureDetector(
          behavior: HitTestBehavior.opaque,
          onHorizontalDragEnd: (r) async {
            if (r.primaryVelocity! > 100) {
              await _goPrev();
            } else if (r.primaryVelocity! < -110) {
              await _goNext();
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
            selectionControls: DictionarySelectionControls(
              selection: _currentSelection,
            ),
            child: SizedBox.expand(
              child: SmoothScroll(
                initialOffset: widget.book.position.offset,
                forceOffsetChangeNotifier: _offsetNotifier,
                onScrollEnd: (offset, maxScrollExtent) async {
                  await widget.db.updateProgress(
                    id: widget.book.uuid,
                    chapter: _chapter,
                    offset: offset,
                  );

                  _offset = offset;
                },
                children: [
                  LayoutBuilder(builder: (context, layout) {
                    return html.Html.fromDom(
                      document: _htmlDocument!,
                      onLinkTap: (url, ctx, s, element) async {
                        await _goUrl(url!);
                      },
                      style: {
                        "p": html.Style.fromTextStyle(
                          _textStyle().copyWith(
                            fontSize: _fontSize,
                            fontWeight: FontWeight.w300,
                          ),
                        ).merge(
                          html.Style(
                            // TODO: Setting
                            margin: const EdgeInsets.only(bottom: 32),
                            textAlign: TextAlign.justify,
                          ),
                        ),
                        // TODO
                        "h2": html.Style(fontSize: const html.FontSize(32)),
                        "html": html.Style(
                          padding: EdgeInsets.symmetric(
                            horizontal:
                                (_paddingPercent / 100) * (layout.maxWidth / 2),
                          ),
                        ),
                      },
                      customRender: {
                        "svg": (context, element) {
                          debugPrint('svg, rendering red box');

                          return Container(
                              height: 30, width: 30, color: Colors.red);
                        }
                      },
                      customImageRenders: {
                        (context, element) {
                          return true;
                        }: (context, ctx, element) {
                          final path = element!.attributes['src']!;

                          if (_images.containsKey(path)) {
                            return Center(child: Image.memory(_images[path]!));
                          }

                          final future = _document!.getResource(path: path);

                          future.then(
                            (value) => setState(() {
                              _images[path] = value;
                            }),
                          );

                          return FutureBuilder(
                            future: future,
                            builder: (ctx, AsyncSnapshot<Uint8List> snapshot) {
                              if (snapshot.hasData) {
                                return Center(
                                    child: Image.memory(snapshot.data!));
                              }

                              return const SizedBox.shrink();
                            },
                          );
                        },
                      },
                    );
                  }),
                ],
              ),
            ),
          ),
          // TODO: Desktop
          // Align(
          //   alignment: Alignment.topLeft,
          //   child: IconButton(
          //     onPressed: () async {
          //       await _goPrev();
          //     },
          //     icon: AnimatedOpacity(
          //       duration: const Duration(milliseconds: 200),
          //       opacity: _showIcons ? 1 : 0,
          //       child: const Icon(Icons.arrow_back_ios),
          //     ),
          //   ),
          // ),
          // Align(
          //   alignment: Alignment.topRight,
          //   child: IconButton(
          //     onPressed: () async {
          //       await _goNext();
          //     },
          //     icon: AnimatedOpacity(
          //       duration: const Duration(milliseconds: 200),
          //       opacity: _showIcons ? 1 : 0,
          //       child: const Icon(Icons.arrow_forward_ios),
          //     ),
          //   ),
          // ),
        ),
      ],
    );
  }

  Future<void> _getAndSetContent() async {
    final content = await _document!.getContent();

    _setContent(content, offset: widget.book.position.offset);
  }

  void _setContent(ContentBlock content, {double offset = 0}) async {
    final doc = dom.Document.html(
        '${content.content}\n${content.contentType.when(html: (extraCss) => extraCss)}');
    setState(() {
      _htmlDocument = doc;
      _offset = offset;
      _chapter = content.chapter;
    });
  }

  void _resetOffset() {
    _offsetNotifier.value = 1;
    _offsetNotifier.value = 0;
  }

  Future<void> _goNext() async {
    final value = await _document!.goNext();

    _setContent(value);
    _resetOffset();

    await widget.db
        .updateProgress(id: widget.book.uuid, chapter: _chapter, offset: 0);
  }

  Future<void> _goPrev() async {
    final value = await _document!.goPrev();

    _setContent(value);
    _resetOffset();

    await widget.db
        .updateProgress(id: widget.book.uuid, chapter: _chapter, offset: 0);
  }

  Future<void> _goUrl(String url) async {
    // TODO try catch url parse
    final content = await _document!.goUrl(url: Uri.parse(url).path);

    final doc = dom.Document.html(
        '${content.content.content}\n${content.content.contentType.when(html: (extraCss) => extraCss)}');
    setState(() {
      _chapter = content.chapter;
      _htmlDocument = doc;
      _offset = 0;
    });

    _resetOffset();
  }

  List<Widget> _actions() {
    return [
      // ValueListenableBuilder(
      //   valueListenable: _isSyncing,
      //   builder: (context, bool syncing, _) {
      // return IconButton(
      //   onPressed: syncing
      //       ? null
      //       : () async {//
      //           final name = (await Pref.currentUserName.value())!;
      //
      //           // TODO try catch
      //           await utilsApi.uploadFile(
      //             repo: "sync",
      //             uuid: widget.book.uuid,
      //             user: GithubUser(login: name, id: id),
      //           );
      //         },
      //   icon: const Icon(Icons.sync),
      //   );
      // },
      // ),
      PopupMenuButton(
        icon: const Icon(Icons.toc),
        position: PopupMenuPosition.under,
        itemBuilder: (context) => <PopupMenuEntry<dynamic>>[
          PopupMenuWidget(
            child: StatefulBuilder(builder: (context, setInnerState) {
              return FutureHandledBuilder(
                future: _document!.getToc(),
                builder: (ctx, List<TocEntry> value) {
                  return Column(
                    children: value.map((e) {
                      return Padding(
                        padding: const EdgeInsets.all(8.0),
                        child: Material(
                          borderRadius: BorderRadius.circular(3),
                          type: MaterialType.card,
                          elevation: 3,
                          child: ListTile(
                            onTap: () async {
                              await _goUrl(e.url);

                              if (!mounted) return;

                              Navigator.of(context).pop();
                            },
                            title: Center(child: Text(e.label)),
                          ),
                        ),
                      );
                    }).toList(),
                  );
                },
              );
            }),
          ),
        ],
      ),
      PopupMenuButton(
        itemBuilder: (context) => <PopupMenuEntry<dynamic>>[
          PopupMenuWidget(
            child: StatefulBuilder(builder: (context, setInnerState) {
              return Column(
                children: [
                  _settingItem(
                    ListTile(
                      contentPadding: const EdgeInsets.symmetric(
                          vertical: 8, horizontal: 16),
                      title: Center(
                          child:
                              Text('Font size: ${_fontSize.roundToDouble()}')),
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
                  _settingItem(
                    ListTile(
                      contentPadding: const EdgeInsets.symmetric(
                          vertical: 8, horizontal: 16),
                      title: Center(
                          child: Text(
                              'Padding: ${_paddingPercent.roundToDouble()}%')),
                      subtitle: Slider(
                        value: _paddingPercent.roundToDouble(),
                        max: 100,
                        onChanged: (val) {
                          setInnerState(() {
                            _paddingPercent = val;
                          });
                        },
                        onChangeEnd: (val) {
                          setState(() {
                            setInnerState(() {
                              _paddingPercent = val;
                            });
                          });
                        },
                      ),
                    ),
                  ),
                  _settingItem(
                    ListTile(
                      contentPadding: const EdgeInsets.symmetric(
                          vertical: 8, horizontal: 16),
                      title: const Center(child: Text('Fonts')),
                      onTap: () async {
                        final result = await showSearch(
                            context: context, delegate: FontSearch());

                        setState(() {
                          _textStyle = GoogleFonts.asMap()[result!]!;
                        });

                        if (!mounted) return;

                        Navigator.of(context).pop();
                      },
                    ),
                  ),
                ],
              );
            }),
          ),
        ],
      )
    ];
  }

  Widget _settingItem(Widget child) {
    return Padding(
      padding: const EdgeInsets.symmetric(horizontal: 2.0),
      child: Card(
        elevation: 4,
        child: child,
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
                    color: Theme.of(context).colorScheme.primaryContainer,
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
                    future: utilsApi.getDefinition(word: selection.value),
                    loadingBuilder: (context) => const SizedBox(
                        height: 100,
                        child: Center(child: CircularProgressIndicator())),
                    errorBuilder: (context, error) =>
                        Text('Unable to fetch word: $error'),
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

class FontSearch extends SearchDelegate<String> {
  static final fonts = GoogleFonts.asMap().keys.toList();
  List<String>? previous;

  @override
  List<Widget>? buildActions(BuildContext context) {
    return null;
  }

  @override
  Widget? buildLeading(BuildContext context) {
    return null;
  }

  @override
  Widget buildResults(BuildContext context) {
    return Container();
  }

  @override
  Widget buildSuggestions(BuildContext context) {
    Widget listView(List<String> listToShow) {
      return ListView.builder(
        itemCount: listToShow.length,
        itemBuilder: (_, i) {
          var font = listToShow[i];
          return ListTile(
            title: Text(font),
            onTap: () {
              close(context, font);
            },
          );
        },
      );
    }

    if (query.isNotEmpty) {
      final result = utilsApi.fontSearch(query: query).then(
        (value) {
          previous = value.toList();

          return value;
        },
      );

      return FutureHandledBuilder(
        future: result,
        builder: (context, Iterable<String> fonts) {
          return listView(
            fonts.toList(),
          );
        },
        loadingBuilder: (context) {
          if (previous != null) {
            return listView(previous!);
          } else {
            return const Center(child: CircularProgressIndicator());
          }
        },
      );
    } else {
      return listView(fonts);
    }
  }
}
