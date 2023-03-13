import 'package:cornered/common/common_page.dart';
import 'package:cornered/gen/ffi.dart';
import 'package:cornered/views/reader.dart';
import 'package:file_picker/file_picker.dart';
import 'package:flutter/material.dart';
import 'package:path_provider/path_provider.dart';

class Library extends StatefulWidget {
  const Library({Key? key}) : super(key: key);

  @override
  State<Library> createState() => _LibraryState();
}

class _LibraryState extends State<Library> {
  List<Book>? _books;

  @override
  void initState() {
    super.initState();

    _initBooks();
  }

  Future<void> _initBooks() async {
    final directory = await getApplicationDocumentsDirectory();

    final db = await api.initDb(path: directory.path);

    final books = await db.getBooks();

    setState(() {
      _books = books;
    });
  }

  Future<void> _add(String path) async {
    final directory = await getApplicationDocumentsDirectory();

    final db = await api.initDb(path: directory.path);

    final books = await db.add(path: path);

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

  Widget _bookItem(Book book) {
    return Padding(
      padding: const EdgeInsets.all(8.0),
      child: Card(
        child: ListTile(
          contentPadding: const EdgeInsets.symmetric(vertical: 8, horizontal: 16),
          title: Text(book.uuid),
          subtitle: Text(book.path),
          onTap: () => Navigator.of(context).push(
            MaterialPageRoute(
              builder: (context) => Reader(path: book.path),
            ),
          ),
        ),
      ),
    );
  }
}
