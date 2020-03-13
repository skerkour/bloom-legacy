import 'package:bloom/ui/const.dart';
import 'package:bloom/ui/notes/views/notes.dart';
import 'package:flutter/material.dart';

class GroupsDrawer extends StatefulWidget {
  const GroupsDrawer({Key key}) : super(key: key);

  @override
  _GroupsDrawerState createState() => _GroupsDrawerState();
}

class _GroupsDrawerState extends State<GroupsDrawer> {
  @override
  Widget build(BuildContext context) {
    return Drawer(
      child: ListView(
        children: <Widget>[
          ListTile(
            leading: const Icon(Icons.group),
            title: const Text('Members'),
            onTap: () {
              Navigator.pop(context);
              Navigator.pushNamedAndRemoveUntil(
                context,
                PATH_GROUPS + '/members',
                (Route<dynamic> route) => route.settings.name == PATH_GROUPS,
              );
            },
          ),
          ListTile(
            leading: const Icon(Icons.settings),
            title: const Text('Preferences'),
            onTap: () {
              Navigator.pop(context);
              Navigator.pushNamedAndRemoveUntil(
                context,
                PATH_GROUPS + '/preferences',
                (Route<dynamic> route) =>
                    route.settings.name == PATH_GROUPS + '/members',
              );
            },
          ),
        ],
      ),
    );
  }
}
