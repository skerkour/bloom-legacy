import 'package:bloom/ui/const.dart';
import 'package:flutter/material.dart';

class PreferencesDrawer extends StatefulWidget {
  const PreferencesDrawer({Key key}) : super(key: key);

  @override
  _PreferencesDrawerState createState() => _PreferencesDrawerState();
}

class _PreferencesDrawerState extends State<PreferencesDrawer> {
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
            leading: const Icon(Icons.palette),
            title: const Text('Theme'),
            onTap: () {
              Navigator.pop(context);
              Navigator.pushNamedAndRemoveUntil(
                context,
                PATH_PREFERENCES,
                (Route<dynamic> route) => route.settings.name == '/',
              );
            },
          ),
        ],
      ),
    );
  }
}
