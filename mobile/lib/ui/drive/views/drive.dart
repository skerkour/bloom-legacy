import 'package:bloom/ui/drive/widgets/drawer.dart';
import 'package:bloom/libs/filesize.dart';
import 'package:flutter/material.dart';

class DriveView extends StatefulWidget {
  const DriveView();

  @override
  _DriveState createState() => _DriveState();
}

class _DriveState extends State<DriveView> {
  @override
  Widget build(BuildContext context) {
    return Scaffold(
      drawer: const DriveDrawer(),
      appBar: AppBar(
        title: const Text('Drive'),
      ),
      body: _buildBody(),
      floatingActionButton: _buildFloatingActionButton(),
    );
  }

  Widget _buildBody() {
    final List<File> files = File.getFiles();

    return Container(
      child: ListView.separated(
          separatorBuilder: (BuildContext context, int index) =>
              const Divider(),
          itemCount: files.length,
          itemBuilder: (BuildContext context, int index) {
            final File file = files[index];

            return ListTile(
              leading: Icon(Icons.movie),
              title: Text(file.name),
              subtitle: Text(filesize(file.size)),
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

class File {
  File({this.name, this.size}) {
    updatedAt = DateTime.now();
  }

  DateTime updatedAt;
  String name;
  int size;

  static List<File> getFiles() {
    return <File>[
      File(name: 'My super clip', size: 42000000),
      File(name: 'My super clip 2', size: 42000000),
    ];
  }
}
