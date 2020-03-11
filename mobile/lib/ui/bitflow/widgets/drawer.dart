import 'package:bloom/ui/bitflow/views/history.dart';
import 'package:bloom/ui/const.dart';
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
            leading: const Icon(Icons.arrow_back),
            title: const Text('Back'),
            onTap: () {
              Navigator.of(context).popUntil(
                  (Route<dynamic> route) => route.settings.name == '/');
            },
          ),
          const Divider(),
          ListTile(
            leading: const Icon(Icons.cloud_download),
            title: const Text('Downloads'),
            onTap: () {
              Navigator.pop(context);
              Navigator.pushNamedAndRemoveUntil(
                context,
                PATH_BITFLOW,
                (Route<dynamic> route) => route.settings.name == '/',
              );
            },
          ),
          ListTile(
            leading: const Icon(Icons.history),
            title: const Text('History'),
            onTap: () {
              Navigator.pop(context);
              Navigator.pushAndRemoveUntil<dynamic>(
                context,
                MaterialPageRoute<dynamic>(
                  builder: (BuildContext context) => const BitflowHistoryView(),
                ),
                (Route<dynamic> route) => route.settings.name == PATH_BITFLOW,
              );
            },
          ),
        ],
      ),
    );
  }
}
