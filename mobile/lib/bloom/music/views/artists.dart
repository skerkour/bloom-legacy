import 'package:bloom/bloom/music/widgets/drawer.dart';
import 'package:flutter/material.dart';

class MusicArtistsView extends StatefulWidget {
  const MusicArtistsView();

  @override
  _MusicArtistsState createState() => _MusicArtistsState();
}

class _MusicArtistsState extends State<MusicArtistsView> {
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
    final List<Artist> artists = Artist.getArtists();

    return Container(
      child: ListView.separated(
          separatorBuilder: (BuildContext context, int index) =>
              const Divider(),
          itemCount: artists.length,
          itemBuilder: (BuildContext context, int index) {
            final Artist file = artists[index];

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

class Artist {
  Artist({this.name, this.size}) {
    updatedAt = DateTime.now();
  }

  DateTime updatedAt;
  String name;
  int size;

  static List<Artist> getArtists() {
    return <Artist>[
      Artist(name: 'My super artist', size: 120),
      Artist(name: 'My super artist 2', size: 12),
    ];
  }
}
