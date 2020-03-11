import 'package:bloom/ui/bitflow/widgets/drawer.dart';
import 'package:flutter/material.dart';

class BitflowHistoryView extends StatefulWidget {
  const BitflowHistoryView();

  @override
  _BitflowHistoryState createState() => _BitflowHistoryState();
}

class _BitflowHistoryState extends State<BitflowHistoryView> {
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
              subtitle: Text('${download.state}'),
            );
          }),
    );
  }

  FloatingActionButton _buildFloatingActionButton() {
    return FloatingActionButton(
      onPressed: () => debugPrint('Delete history pressed'),
      child: const Icon(Icons.delete),
      backgroundColor: Colors.red,
    );
  }
}

class Download {
  Download({this.state, this.name});

  String state;
  String name;

  static List<Download> getDownloads() {
    return <Download>[
      Download(state: 'Success', name: 'My super movie'),
    ];
  }
}
