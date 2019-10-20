import 'package:bloom/bloom/bitflow/widgets/drawer.dart';
import 'package:flutter/material.dart';

class BitflowView extends StatefulWidget {
  const BitflowView();

  @override
  _BitflowState createState() => _BitflowState();
}

class _BitflowState extends State<BitflowView> {
  @override
  Widget build(BuildContext context) {
    return Scaffold(
      endDrawer: const BitflowDrawer(),
      appBar: AppBar(
        title: const Text('Bitflow'),
      ),
      body: const Center(child: Text('Bitflow')),
    );
  }
}
