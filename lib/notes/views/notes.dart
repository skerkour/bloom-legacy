import 'package:bloom/kernel/widgets/drawer.dart';
import 'package:flutter/material.dart';

class Notes extends StatefulWidget {
  @override
  _NotesState createState() => _NotesState();
}

class _NotesState extends State<Notes> {
  @override
  Widget build(BuildContext context) {
    return Scaffold(
      drawer: const BlmDrawer(),
      appBar: AppBar(
        // Here we take the value from the MyHomePage object that was created by
        // the App.build method, and use it to set our appbar title.
        title: const Text('Notes'),
      ),
      body: Center(
        // Center is a layout widget. It takes a single child and positions it
        // in the middle of the parent.
        child: const Text('Notes'),
      ),
      floatingActionButton: FloatingActionButton(
        onPressed: () => _newNoteTapped(context),
        child: Icon(Icons.add),
      ),
    );
  }

  void _newNoteTapped(BuildContext ctx) {
    print('new note tapped');
    // "-1" id indicates the note is not new
    // var emptyNote = new Note(-1, "", "", DateTime.now(), DateTime.now(), Colors.white);
    // Navigator.push(ctx,MaterialPageRoute(builder: (ctx) => NotePage(emptyNote)));
  }
}
