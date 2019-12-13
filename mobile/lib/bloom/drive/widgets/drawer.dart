import 'package:flutter/material.dart';

class DriveDrawer extends StatefulWidget {
  const DriveDrawer({Key key}) : super(key: key);

  @override
  _DriveDrawerState createState() => _DriveDrawerState();
}

class _DriveDrawerState extends State<DriveDrawer> {
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
              Navigator.of(context).popUntil(
                  (Route<dynamic> route) => route.settings.name == '/');
            },
          ),
          const Divider(),
          ListTile(
            leading: const Icon(Icons.list),
            title: const Text('Drive'),
            onTap: () => debugPrint('Drive tapped'),
          ),
        ],
      ),
    );
  }
}
