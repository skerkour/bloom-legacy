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
      body: _buildBody(),
    );
  }

  Center _buildBody() {
    return const Center(child: Text('Gallery'));
  }
}

class Photo {
  Photo({
    this.file,
    this.title,
    this.caption,
    this.isFavorite = false,
  });

  final String file;
  final String title;
  final String caption;

  bool isFavorite;
  String get tag => file; // Assuming that all asset names are unique.

  bool get isValid =>
      file != null && title != null && caption != null && isFavorite != null;
}
