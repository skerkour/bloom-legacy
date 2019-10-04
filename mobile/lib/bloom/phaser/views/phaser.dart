import 'package:bloom/bloom/phaser/widgets/drawer.dart';
import 'package:flutter/material.dart';

class PhaserView extends StatefulWidget {
  const PhaserView();

  @override
  _PhaserState createState() => _PhaserState();
}

class _PhaserState extends State<PhaserView> {
  @override
  Widget build(BuildContext context) {
    return Scaffold(
      endDrawer: const PhaserDrawer(),
      appBar: AppBar(
        title: const Text('Phaser'),
      ),
      body: const Center(child: Text('Phaser')),
    );
  }
}
