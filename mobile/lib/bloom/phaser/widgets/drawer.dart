import 'package:flutter/material.dart';

class PhaserDrawer extends StatefulWidget {
  const PhaserDrawer({Key key}) : super(key: key);

  @override
  _PhaserDrawerState createState() => _PhaserDrawerState();
}

class _PhaserDrawerState extends State<PhaserDrawer> {
  @override
  Widget build(BuildContext context) {
    return Drawer(
      child: ListView(
        children: <Widget>[
          ListTile(
            leading: const Icon(Icons.list),
            title: const Text('Scans'),
            onTap: () => debugPrint('Scans tapped'),
          ),
        ],
      ),
    );
  }
}
