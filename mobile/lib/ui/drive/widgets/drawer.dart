import 'package:bloom/ui/const.dart';
import 'package:bloom/ui/drive/views/trash.dart';
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
              Navigator.of(context).popUntil(
                  (Route<dynamic> route) => route.settings.name == '/');
            },
          ),
          const Divider(),
          ListTile(
            leading: const Icon(Icons.cloud),
            title: const Text('Drive'),
            onTap: () {
              Navigator.pop(context);
              Navigator.pushNamedAndRemoveUntil(
                context,
                PATH_DRIVE,
                (Route<dynamic> route) => route.settings.name == '/',
              );
            },
          ),
          ListTile(
            leading: const Icon(Icons.delete),
            title: const Text('Trash'),
            onTap: () {
              Navigator.pop(context);
              Navigator.pushAndRemoveUntil<dynamic>(
                context,
                MaterialPageRoute<dynamic>(
                  builder: (BuildContext context) => const DriveTrashView(),
                ),
                (Route<dynamic> route) => route.settings.name == PATH_DRIVE,
              );
            },
          ),
        ],
      ),
    );
  }
}
