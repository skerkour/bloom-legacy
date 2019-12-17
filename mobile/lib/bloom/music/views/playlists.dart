import 'package:bloom/bloom/music/widgets/drawer.dart';
import 'package:flutter/material.dart';

class MusicPlaylistsView extends StatefulWidget {
  const MusicPlaylistsView();

  @override
  _MusicPlaylistsState createState() => _MusicPlaylistsState();
}

class _MusicPlaylistsState extends State<MusicPlaylistsView> {
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
    final List<Playlist> files = Playlist.getPlaylists();

    return Container(
      child: ListView.separated(
          separatorBuilder: (BuildContext context, int index) =>
              const Divider(),
          itemCount: files.length,
          itemBuilder: (BuildContext context, int index) {
            final Playlist file = files[index];

            return ListTile(
              leading: Icon(Icons.queue_music),
              title: Text(file.name),
              subtitle: Text('${file.size} songs'),
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

class Playlist {
  Playlist({this.name, this.size}) {
    updatedAt = DateTime.now();
  }

  DateTime updatedAt;
  String name;
  int size;

  static List<Playlist> getPlaylists() {
    return <Playlist>[
      Playlist(name: 'My super playlist', size: 120),
      Playlist(name: 'My super playlist 2', size: 12),
    ];
  }
}
