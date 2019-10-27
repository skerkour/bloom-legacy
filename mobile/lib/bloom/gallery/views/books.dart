import 'package:bloom/bloom/gallery/widgets/drawer.dart';
import 'package:flutter/material.dart';

class GalleryView extends StatefulWidget {
  const GalleryView();

  @override
  _GalleryState createState() => _GalleryState();
}

class _GalleryState extends State<GalleryView> {
  @override
  Widget build(BuildContext context) {
    return Scaffold(
      drawer: const GalleryDrawer(),
      appBar: AppBar(
        title: const Text('Gallery'),
      ),
      body: const Center(child: Text('Gallery')),
    );
  }
}
