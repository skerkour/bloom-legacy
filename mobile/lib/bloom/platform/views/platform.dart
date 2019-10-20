import 'package:bloom/bloom/platform/widgets/drawer.dart';
import 'package:flutter/material.dart';

class PlatformView extends StatefulWidget {
  const PlatformView();

  @override
  _PlatformState createState() => _PlatformState();
}

class _PlatformState extends State<PlatformView> {
  @override
  Widget build(BuildContext context) {
    return Scaffold(
      endDrawer: const PlatformDrawer(),
      appBar: AppBar(
        title: const Text('Platform'),
      ),
      body: const Center(child: Text('Platform')),
    );
  }
}
