import 'package:bloom/bloom/gallery/widgets/drawer.dart';
import 'package:flutter/material.dart';

class GalleryAlbumsView extends StatefulWidget {
  const GalleryAlbumsView();

  @override
  _GalleryAlbumsState createState() => _GalleryAlbumsState();
}

class _GalleryAlbumsState extends State<GalleryAlbumsView> {
  @override
  Widget build(BuildContext context) {
    return Scaffold(
      drawer: const GalleryDrawer(),
      appBar: AppBar(
        title: const Text('Gallery'),
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
            final Album album = albums[index];

            return ListTile(
              leading: Icon(Icons.photo_album),
              title: Text(album.name),
              subtitle: Text('${album.size} media'),
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
      Album(name: 'My super Album', size: 120),
      Album(name: 'My super Album 2', size: 12),
    ];
  }
}
