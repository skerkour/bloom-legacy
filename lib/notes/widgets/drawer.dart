import 'package:flutter/material.dart';

class NotesDrawer extends StatefulWidget {
  const NotesDrawer({Key key}) : super(key: key);

  @override
  _NotesDrawerState createState() => _NotesDrawerState();
}

class _NotesDrawerState extends State<NotesDrawer> {
  @override
  Widget build(BuildContext context) {
    return Container(
      child: const Text('Settings Notes'),
    );
  }
}
