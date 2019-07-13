import 'package:bloom/kernel/widgets/drawer.dart';
import 'package:bloom/notes/models/db/note.dart';
import 'package:bloom/notes/views/note.dart';
import 'package:flutter/material.dart';

class NotesView extends StatefulWidget {
  @override
  _NotesState createState() => _NotesState();
}

class _NotesState extends State<NotesView> {
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
        backgroundColor: Colors.red,
      ),
    );
  }

  void _newNoteTapped(BuildContext ctx) {
    print('new note tapped');
    Navigator.push<dynamic>(ctx, MaterialPageRoute<dynamic>(builder: (BuildContext ctx) => NoteView(Note())));
    // "-1" id indicates the note is not new
    // var emptyNote = new Note(-1, "", "", DateTime.now(), DateTime.now(), Colors.white);
    // Navigator.push(ctx,MaterialPageRoute(builder: (ctx) => NotePage(emptyNote)));
  }
}
