import 'package:flutter/material.dart';

class MusicDrawer extends StatefulWidget {
  const MusicDrawer({Key key}) : super(key: key);

  @override
  _MusicDrawerState createState() => _MusicDrawerState();
}

class _MusicDrawerState extends State<MusicDrawer> {
  @override
  Widget build(BuildContext context) {
    return Drawer(
      child: ListView(
        children: <Widget>[
          ListTile(
            leading: const Icon(Icons.arrow_back),
            title: const Text('Back'),
            onTap: () {
              Navigator.of(context).pop();
              Navigator.of(context).pop();
            },
          ),
          const Divider(),
          ListTile(
            leading: const Icon(Icons.music_note),
            title: const Text('Music'),
            onTap: () => debugPrint('Music tapped'),
          ),
        ],
      ),
    );
  }
}
