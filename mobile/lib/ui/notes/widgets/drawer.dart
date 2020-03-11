import 'package:bloom/ui/const.dart';
import 'package:bloom/ui/notes/views/notes.dart';
import 'package:flutter/material.dart';

class NotesDrawer extends StatefulWidget {
  const NotesDrawer({Key key}) : super(key: key);

  @override
  _NotesDrawerState createState() => _NotesDrawerState();
}

class _NotesDrawerState extends State<NotesDrawer> {
  @override
  Widget build(BuildContext context) {
    return Drawer(
      child: ListView(
        children: <Widget>[
          ListTile(
            leading: const Icon(Icons.list),
            title: const Text('Notes'),
            onTap: () {
              Navigator.pop(context);
              Navigator.pushNamedAndRemoveUntil(
                context,
                PATH_NOTES,
                (Route<dynamic> route) => route.settings.name == '/',
              );
            },
          ),
          ListTile(
            leading: const Icon(Icons.archive),
            title: const Text('Archive'),
            onTap: () {
              Navigator.pop(context);
              Navigator.pushAndRemoveUntil<dynamic>(
                context,
                MaterialPageRoute<dynamic>(
                  builder: (BuildContext context) =>
                      const NotesView(archive: true),
                ),
                (Route<dynamic> route) => route.settings.name == PATH_NOTES,
              );
            },
          ),
        ],
      ),
    );
  }
}
