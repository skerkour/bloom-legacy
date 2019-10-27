import 'package:bloom/bloom/music/widgets/drawer.dart';
import 'package:flutter/material.dart';

class MusicView extends StatefulWidget {
  const MusicView();

  @override
  _MusicState createState() => _MusicState();
}

class _MusicState extends State<MusicView> {
  @override
  Widget build(BuildContext context) {
    return Scaffold(
      drawer: const MusicDrawer(),
      appBar: AppBar(
        title: const Text('Music'),
      ),
      body: const Center(child: Text('Music')),
    );
  }
}
