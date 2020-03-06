import 'package:bloom/bloom/books/widgets/drawer.dart';
import 'package:flutter/material.dart';

class BooksView extends StatefulWidget {
  const BooksView();

  @override
  _BooksState createState() => _BooksState();
}

class _BooksState extends State<BooksView> {
  @override
  Widget build(BuildContext context) {
    return Scaffold(
      drawer: const BooksDrawer(),
      appBar: AppBar(
        title: const Text('Books'),
      ),
      body: const Center(child: Text('Books')),
    );
  }
}
