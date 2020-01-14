import 'package:bloom/bloom/music/widgets/drawer.dart';
import 'package:flutter/material.dart';

class MusicAlbumsView extends StatefulWidget {
  const MusicAlbumsView();

  @override
  _MusicAlbumsState createState() => _MusicAlbumsState();
}

class _MusicAlbumsState extends State<MusicAlbumsView> {
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
    final List<Album> albums = Album.getAlbums();

    return Container(
      child: ListView.separated(
          separatorBuilder: (BuildContext context, int index) =>
              const Divider(),
          itemCount: albums.length,
          itemBuilder: (BuildContext context, int index) {
            final Album file = albums[index];

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

class Album {
  Album({this.name, this.size}) {
    updatedAt = DateTime.now();
  }

  DateTime updatedAt;
  String name;
  int size;

  static List<Album> getAlbums() {
    return <Album>[
      Album(name: 'My super album', size: 120),
      Album(name: 'My super album 2', size: 12),
    ];
  }
}
