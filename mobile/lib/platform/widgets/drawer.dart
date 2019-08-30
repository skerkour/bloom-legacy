import 'package:flutter/material.dart';

class PlatformDrawer extends StatefulWidget {
  const PlatformDrawer({Key key}) : super(key: key);

  @override
  _PlatformDrawerState createState() => _PlatformDrawerState();
}

class _PlatformDrawerState extends State<PlatformDrawer> {
  @override
  Widget build(BuildContext context) {
    return Drawer(
      child: ListView(
        children: <Widget>[
          ListTile(
            leading: const Icon(Icons.list),
            title: const Text('Platform'),
            onTap: () => debugPrint('Platform tapped'),
          ),
        ],
      ),
    );
  }
}
