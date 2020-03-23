import 'package:bloom/ui/const.dart';
import 'package:bloom/ui/files/views/trash.dart';
import 'package:flutter/material.dart';

class FilesDrawer extends StatefulWidget {
  const FilesDrawer({Key key}) : super(key: key);

  @override
  _FilesDrawerState createState() => _FilesDrawerState();
}

class _FilesDrawerState extends State<FilesDrawer> {
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
            title: const Text('Files'),
            onTap: () {
              Navigator.pop(context);
              Navigator.pushNamedAndRemoveUntil(
                context,
                PATH_FILES,
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
                  builder: (BuildContext context) => const FilesTrashView(),
                ),
                (Route<dynamic> route) => route.settings.name == PATH_FILES,
              );
            },
          ),
        ],
      ),
    );
  }
}
