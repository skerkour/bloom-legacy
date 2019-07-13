import 'package:flutter/material.dart';

class NotesSettings extends StatefulWidget {
  const NotesSettings({Key key}) : super(key: key);

  @override
  _NotesSettingsState createState() => _NotesSettingsState();
}

class _NotesSettingsState extends State<NotesSettings> {
  @override
  Widget build(BuildContext context) {
    return Container(
       child: const Text('Settings Notes'),
    );
  }
}