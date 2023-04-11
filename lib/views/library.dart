import 'dart:typed_data';

import 'package:cornered/common/common_page.dart';
import 'package:cornered/gen/ffi.dart';
import 'package:cornered/views/reader.dart';
import 'package:cornered/views/settings.dart';
import 'package:file_picker/file_picker.dart';
import 'package:flutter/material.dart';
import 'package:page_transition/page_transition.dart';

class Library extends StatefulWidget {
  const Library({Key? key}) : super(key: key);

  @override
  State<Library> createState() => _LibraryState();
}

class _LibraryState extends State<Library> {
  List<Book>? _books;
  Database? db;

  final Map<String, Widget> _uuidToImage = {};

  List<Book> _selected = [];

  @override
  void initState() {
    super.initState();

    _initBooks();
  }

  Future<void> _initBooks() async {
    db = await booksApi.getDb();

    final books = await db!.getBooks();

    setState(() {
      _books = books;
    });
  }

  Future<void> _add(String path) async {
    final books = await db!.addBook(path: path);

    setState(() {
      _books = books;
    });
  }

  @override
  Widget build(BuildContext context) {
    return CommonPage(
      title: 'Library',
      floatingActionButton: FloatingActionButton(
        onPressed: () async {
          final file = await FilePicker.platform.pickFiles();

          for (final file in file!.files) {
            await _add(file.path!);
          }
        },
        child: const Icon(Icons.add),
      ),
      appBar: _isSelecting() ? _appBarSelecting() : null,
      actions: [
        IconButton(
          onPressed: () async {
            Navigator.push(
              context,
              PageTransition(child: const Settings(), type: PageTransitionType.fade),
            );
          },
          icon: const Icon(Icons.settings),
        ),
        // IconButton(
        //   onPressed: () async {
        //     // TODO fallable and not repeat
        //
        //     var id = await Pref.currentUser.value();
        //     if (id == null) {
        //       final response = await utilsApi.auth();
        //
        //       if (!mounted) return;
        //
        //       final fut = utilsApi.poll(ongoing: response);
        //
        //       await showDialog<void>(
        //         context: context,
        //         barrierDismissible: false,
        //         builder: (BuildContext context) {
        //           fut.then((_) => Navigator.of(context).pop());
        //
        //           return AlertDialog(
        //             title: const Text('Code'),
        //             content: SingleChildScrollView(
        //               child: ListBody(
        //                 children: [
        //                   Text('Navigate to ${response.verificationUri}', style: Theme.of(context).textTheme.headlineMedium),
        //                   const SizedBox(height: 16),
        //                   SelectableText('userCode: ${response.userCode}'),
        //                 ],
        //               ),
        //             ),
        //           );
        //         },
        //       );
        //
        //       final user = await fut;
        //
        //       await Pref.currentUser.set(user.id);
        //       await Pref.currentUserName.set(user.login);
        //
        //       id = user.id;
        //     }
        //
        //     final name = await Pref.currentUserName.value();
        //
        //     final files = await utilsApi.updateFiles(
        //       repo: "sync",
        //       user: GithubUser(login: name!, id: id),
        //     );
        //
        //     await _initBooks();
        //   },
        //   icon: const Icon(Icons.download),
        // ),
      ],
      child: _body(),
    );
  }

  Widget _body() {
    if (_books == null) {
      return const Center(child: CircularProgressIndicator());
    }

    return RefreshIndicator(
      onRefresh: () async {
        await _initBooks();

        return Future.value();
      },
      child: ListView(
        physics: const AlwaysScrollableScrollPhysics(),
        children: _books!.map((book) => _bookItem(book)).toList(),
      ),
    );
  }

  Future<Meta> getMeta(Book book) async {
    try {
      final meta = await booksApi.getMeta(id: book.uuid);

      return meta;
    } catch (e) {
      debugPrint(e.toString());
      rethrow;
    }
  }

  Widget _bookItem(Book book) {
    return FutureBuilder(
      future: getMeta(book),
      builder: (context, AsyncSnapshot<Meta> snapshot) {
        if (!snapshot.hasData) {
          return const Center(child: CircularProgressIndicator());
        }

        return Padding(
          padding: const EdgeInsets.all(8.0),
          child: Card(
            child: ListTile(
              selected: _selected.contains(book),
              selectedTileColor: Theme.of(context).colorScheme.secondary.withOpacity(0.2),
              contentPadding: const EdgeInsets.symmetric(vertical: 16, horizontal: 16),
              title: Text(snapshot.data!.title ?? ''),
              subtitle: snapshot.data?.author != null ? Text(snapshot.data!.author!) : null,
              onLongPress: () {
                _selectTapBook(book);
              },
              leading: _selected.contains(book)
                  ? const CircleAvatar(
                      backgroundColor: Colors.white,
                      child: Icon(Icons.check),
                    )
                  : snapshot.data!.cover != null
                      ? _uuidToImage[book.uuid] ?? _setCover(snapshot.data!.cover!, book.uuid)
                      : null,
              onTap: () async {
                if (_isSelecting()) {
                  _selectTapBook(book);

                  return;
                }

                final result = await Navigator.of(context).push(
                  PageTransition(
                    duration: const Duration(milliseconds: 100),
                    child: Reader(
                      book: book,
                      db: db!,
                    ),
                    type: PageTransitionType.fade,
                  ),
                );

                if (!mounted) return;

                final index = _books!.indexWhere((element) => element.uuid == result!.uuid);

                setState(() {
                  _books?[index] = result!;
                });
              },
            ),
          ),
        );
      },
    );
  }

  Widget _setCover(Uint8List cover, String uuid) {
    _uuidToImage[uuid] = Image.memory(cover);

    return _uuidToImage[uuid]!;
  }

  bool _isSelecting() {
    return _selected.isNotEmpty;
  }

  void _selectTapBook(Book book) {
    if (_selected.contains(book)) {
      // deselect
      setState(() {
        _selected.remove(book);
      });
    } else {
      // select
      setState(() {
        _selected.add(book);
      });
    }
  }

  AppBar _appBarSelecting() {
    return AppBar(
      title: Text('${_selected.length} selected'),
      actions: [
        IconButton(
          onPressed: () async {
            // await db!.deleteBooks(_selected);

            setState(() {
              _books!.removeWhere((element) => _selected.contains(element));
              _selected = [];
            });
          },
          icon: const Icon(Icons.delete),
        ),
      ],
    );
  }
}
