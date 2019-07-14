import 'package:bloom/notes/views/archive.dart';
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
      child: ListView(
        children: <Widget>[
          InkWell(
            onTap: () => Navigator.pushNamed(context, '/notes'),
            child: Padding(
              padding: const EdgeInsets.all(4.0),
              child: Row(
                children: <Widget>[
                  Icon(
                    Icons.list,
                    color: Colors.black,
                  ),
                  const SizedBox(width: 12),
                  const Text('Notes')
                ],
              ),
            ),
          ),
          InkWell(
            onTap: () => Navigator.push<dynamic>(
              context,
              MaterialPageRoute<dynamic>(
                builder: (BuildContext context) => ArchiveView(),
              ),
            ),
            child: Padding(
              padding: const EdgeInsets.all(4.0),
              child: Row(
                children: <Widget>[
                  Icon(
                    Icons.archive,
                    color: Colors.black,
                  ),
                  const SizedBox(width: 12),
                  const Text('Archive')
                ],
              ),
            ),
          ),
        ],
      ),
    );
  }
}
