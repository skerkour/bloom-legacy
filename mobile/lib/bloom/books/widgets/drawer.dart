import 'package:flutter/material.dart';

class BooksDrawer extends StatefulWidget {
  const BooksDrawer({Key key}) : super(key: key);

  @override
  _BooksDrawerState createState() => _BooksDrawerState();
}

class _BooksDrawerState extends State<BooksDrawer> {
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
            leading: const Icon(Icons.book),
            title: const Text('Books'),
            onTap: () => debugPrint('Books tapped'),
          ),
        ],
      ),
    );
  }
}
