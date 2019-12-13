import 'package:bloom/bloom/phaser/widgets/drawer.dart';
import 'package:flutter/material.dart';

class PhaserScansView extends StatefulWidget {
  const PhaserScansView();

  @override
  _PhaserScansState createState() => _PhaserScansState();
}

class _PhaserScansState extends State<PhaserScansView> {
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
