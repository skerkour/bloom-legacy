import 'package:bloom/bloom/const.dart';
import 'package:bloom/bloom/gallery/views/albums.dart';
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
              Navigator.of(context).popUntil(
                  (Route<dynamic> route) => route.settings.name == '/');
            },
          ),
          const Divider(),
          ListTile(
            leading: const Icon(Icons.photo_library),
            title: const Text('Galley'),
            onTap: () {
              Navigator.pop(context);
              Navigator.pushNamedAndRemoveUntil(
                context,
                PATH_GALLERY,
                (Route<dynamic> route) => route.settings.name == '/',
              );
            },
          ),
          ListTile(
            leading: const Icon(Icons.photo_album),
            title: const Text('Albums'),
            onTap: () {
              Navigator.pop(context);
              Navigator.pushAndRemoveUntil<dynamic>(
                context,
                MaterialPageRoute<dynamic>(
                  builder: (BuildContext context) => const GalleryAlbumsView(),
                ),
                (Route<dynamic> route) => route.settings.name == PATH_GALLERY,
              );
            },
          ),
        ],
      ),
    );
  }
}
