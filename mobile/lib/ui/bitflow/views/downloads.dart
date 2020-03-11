import 'package:bloom/ui/bitflow/widgets/drawer.dart';
import 'package:flutter/material.dart';

class BitflowDownloadsView extends StatefulWidget {
  const BitflowDownloadsView();

  @override
  _BitflowDownloadsState createState() => _BitflowDownloadsState();
}

class _BitflowDownloadsState extends State<BitflowDownloadsView> {
  @override
  Widget build(BuildContext context) {
    return Scaffold(
      drawer: const BitflowDrawer(),
      appBar: AppBar(
        title: const Text('Bitflow'),
      ),
      body: _buildBody(),
      floatingActionButton: _buildFloatingActionButton(),
    );
  }

  Widget _buildBody() {
    final List<Download> downloads = Download.getDownloads();

    return Container(
      child: ListView.separated(
          separatorBuilder: (BuildContext context, int index) =>
              const Divider(),
          itemCount: downloads.length,
          itemBuilder: (BuildContext context, int index) {
            final Download download = downloads[index];

            return ListTile(
              leading: Icon(Icons.movie),
              title: Text(download.name),
              subtitle: Text('${download.progress}%'),
              trailing: Icon(Icons.more_vert),
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

class Download {
  Download({this.type, this.name, this.progress});

  String type;
  String name;
  int progress;

  static List<Download> getDownloads() {
    return <Download>[
      Download(type: 'Torrent', name: 'My super movie', progress: 42),
    ];
  }
}
