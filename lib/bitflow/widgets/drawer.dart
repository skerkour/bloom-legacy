import 'package:flutter/material.dart';

class BitflowDrawer extends StatefulWidget {
  const BitflowDrawer({Key key}) : super(key: key);

  @override
  _BitflowDrawerState createState() => _BitflowDrawerState();
}

class _BitflowDrawerState extends State<BitflowDrawer> {
  @override
  Widget build(BuildContext context) {
    return Drawer(
      child: ListView(
        children: <Widget>[
          ListTile(
            leading: const Icon(Icons.list),
            title: const Text('Bitflow'),
            onTap: () => debugPrint('Bitflow tapped'),
          ),
        ],
      ),
    );
  }
}
