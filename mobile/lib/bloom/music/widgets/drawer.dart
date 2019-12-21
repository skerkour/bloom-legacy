import 'package:bloom/bloom/const.dart';
import 'package:bloom/bloom/music/views/albums.dart';
import 'package:bloom/bloom/music/views/artists.dart';
import 'package:bloom/bloom/music/views/playlists.dart';
import 'package:flutter/material.dart';

class MusicDrawer extends StatefulWidget {
  const MusicDrawer({Key key}) : super(key: key);

  @override
  _MusicDrawerState createState() => _MusicDrawerState();
}

class _MusicDrawerState extends State<MusicDrawer> {
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
            leading: const Icon(Icons.music_note),
            title: const Text('Songs'),
            onTap: () {
              Navigator.pop(context);
              Navigator.pushNamedAndRemoveUntil(
                context,
                PATH_MUSIC,
                (Route<dynamic> route) => route.settings.name == '/',
              );
            },
          ),
          ListTile(
            leading: const Icon(Icons.library_music),
            title: const Text('Artists'),
            onTap: () {
              Navigator.pop(context);
              Navigator.pushAndRemoveUntil<dynamic>(
                context,
                MaterialPageRoute<dynamic>(
                  builder: (BuildContext context) => const MusicArtistsView(),
                ),
                (Route<dynamic> route) => route.settings.name == PATH_MUSIC,
              );
            },
          ),
          ListTile(
            leading: const Icon(Icons.music_video),
            title: const Text('Albums'),
            onTap: () {
              Navigator.pop(context);
              Navigator.pushAndRemoveUntil<dynamic>(
                context,
                MaterialPageRoute<dynamic>(
                  builder: (BuildContext context) => const MusicAlbumsView(),
                ),
                (Route<dynamic> route) => route.settings.name == PATH_MUSIC,
              );
            },
          ),
          ListTile(
            leading: const Icon(Icons.queue_music),
            title: const Text('Playlists'),
            onTap: () {
              Navigator.pop(context);
              Navigator.pushAndRemoveUntil<dynamic>(
                context,
                MaterialPageRoute<dynamic>(
                  builder: (BuildContext context) => const MusicPlaylistsView(),
                ),
                (Route<dynamic> route) => route.settings.name == PATH_MUSIC,
              );
            },
          ),
        ],
      ),
    );
  }
}
