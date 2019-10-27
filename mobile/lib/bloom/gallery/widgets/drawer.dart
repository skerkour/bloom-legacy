import 'package:flutter/material.dart';

class GalleryDrawer extends StatefulWidget {
  const GalleryDrawer({Key key}) : super(key: key);

  @override
  _GalleryDrawerState createState() => _GalleryDrawerState();
}

class _GalleryDrawerState extends State<GalleryDrawer> {
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
              Navigator.of(context).pop();
            },
          ),
          const Divider(),
          ListTile(
            leading: const Icon(Icons.photo),
            title: const Text('Gallery'),
            onTap: () => debugPrint('Gallery tapped'),
          ),
        ],
      ),
    );
  }
}
