import 'package:bloom/bloom/music/widgets/drawer.dart';
import 'package:flutter/material.dart';

class MusicSongsView extends StatefulWidget {
  const MusicSongsView();

  @override
  _MusicSongsState createState() => _MusicSongsState();
}

class _MusicSongsState extends State<MusicSongsView> {
  @override
  Widget build(BuildContext context) {
    return Scaffold(
      drawer: const MusicDrawer(),
      appBar: AppBar(
        title: const Text('Music'),
      ),
      body: _buildBody(),
      floatingActionButton: _buildFloatingActionButton(),
    );
  }

  Widget _buildBody() {
    final List<Songs> files = Songs.getSongs();

    return Container(
      child: ListView.separated(
          separatorBuilder: (BuildContext context, int index) =>
              const Divider(),
          itemCount: files.length,
          itemBuilder: (BuildContext context, int index) {
            final Songs file = files[index];

            return ListTile(
              leading: Icon(Icons.music_note),
              title: Text(file.name),
              subtitle: Text('${file.artist} - ${file.album}'),
            );
          }),
    );
  }

  FloatingActionButton _buildFloatingActionButton() {
    return FloatingActionButton(
      onPressed: () => debugPrint('Add download pressed'),
      child: Icon(Icons.add),
      backgroundColor: Colors.red,
    );
  }
}

class Songs {
  Songs({this.name, this.album, this.artist});

  String artist;
  String name;
  String album;

  static List<Songs> getSongs() {
    return <Songs>[
      Songs(
          name: 'My super song',
          album: 'tant qu\'on est la',
          artist: 'Hugo TSR'),
      Songs(
          name: 'My super song 2',
          album: 'tant qu\'on est la',
          artist: 'Hugo TSR'),
    ];
  }
}
