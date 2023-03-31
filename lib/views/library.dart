import 'dart:io';

import 'package:cornered/common/common_page.dart';
import 'package:cornered/common/preferences.dart';
import 'package:cornered/gen/ffi.dart';
import 'package:cornered/gen/util_generated.dart';
import 'package:cornered/views/reader.dart';
import 'package:file_picker/file_picker.dart';
import 'package:flutter/foundation.dart';
import 'package:flutter/material.dart';
import 'package:page_transition/page_transition.dart';
import 'package:path_provider/path_provider.dart';

class Library extends StatefulWidget {
  const Library({Key? key}) : super(key: key);

  @override
  State<Library> createState() => _LibraryState();
}

class _LibraryState extends State<Library> {
  List<Book>? _books;
  Database? db;

  @override
  void initState() {
    super.initState();

    _initBooks();
  }

  Future<void> _initBooks() async {
    final directory = await getApplicationDocumentsDirectory();

    db = await booksApi.initDb(path: directory.path);

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
      actions: [
        IconButton(
          onPressed: () async {
            // TODO fallable and not repeat

            var id = await Pref.currentUser.value();
            if (id == null) {
              final response = await utilsApi.auth();

              if (!mounted) return;

              final fut = utilsApi.poll(ongoing: response);

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
                          Text('Navigate to ${response.verificationUri}', style: Theme.of(context).textTheme.headlineMedium),
                          const SizedBox(height: 16),
                          SelectableText('userCode: ${response.userCode}'),
                        ],
                      ),
                    ),
                  );
                },
              );

              final user = await fut;

              await Pref.currentUser.set(user.id);
              await Pref.currentUserName.set(user.login);

              id = user.id;
            }

            final name = await Pref.currentUserName.value();

            final files = await utilsApi.getFiles(
              repo: "sync",
              user: GithubUser(login: name!, id: id),
            );

            for (final undownloaded in files) {
              final url = undownloaded.downloadUrl;

              final httpClient = HttpClient();

              var request = await httpClient.getUrl(Uri.parse(url));
              var response = await request.close();
              var bytes = await consolidateHttpClientResponseBytes(response);
              String dir = (await getApplicationDocumentsDirectory()).path;
              // TODO
              File file = File('$dir/book.epub');
              await file.writeAsBytes(bytes);

              await _add(file.path);
            }
          },
          icon: const Icon(Icons.download),
        ),
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
                contentPadding: const EdgeInsets.symmetric(vertical: 8, horizontal: 16),
                title: Text(snapshot.data!.title ?? ''),
                subtitle: Text(snapshot.data?.author ?? ''),
                leading: snapshot.data!.cover != null ? Image.memory(snapshot.data!.cover!) : null,
                onTap: () async {
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
        });
  }
}
