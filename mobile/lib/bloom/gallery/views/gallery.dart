import 'package:bloom/bloom/const.dart';
import 'package:bloom/bloom/gallery/widgets/drawer.dart';
import 'package:flutter/material.dart';

// see https://github.com/flutter/flutter/blob/master/examples/flutter_gallery/lib/demo/material/grid_list_demo.dart
const double _kMinFlingVelocity = 800.0;

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
      body: _buildBody(context),
    );
  }

  Widget _buildBody(BuildContext context) {
    final List<Media> media = Media.getMedia();

    return GridView.extent(
      maxCrossAxisExtent: 150,
      padding: const EdgeInsets.all(2),
      mainAxisSpacing: 2,
      crossAxisSpacing: 2,
      children: media.map((Media photo) {
        final Widget image = GestureDetector(
          onTap: () {
            showPhoto(context, photo);
          },
          child: Hero(
            key: Key(photo.file),
            tag: photo.file,
            child: Image.asset(
              photo.file,
              fit: BoxFit.cover,
            ),
          ),
        );
        // Container(child: Image.asset(m.file));
        return image;
      }).toList(),
    );
  }

  void showPhoto(BuildContext context, Media photo) {
    Navigator.push(context,
        MaterialPageRoute<void>(builder: (BuildContext context) {
      return Scaffold(
        appBar: AppBar(
          title: Text(photo.file),
        ),
        body: SizedBox.expand(
          child: Hero(
            tag: photo.file,
            child: GridPhotoViewer(photo: photo),
          ),
        ),
      );
    }));
  }
}

class GridPhotoViewer extends StatefulWidget {
  const GridPhotoViewer({Key key, this.photo}) : super(key: key);

  final Media photo;

  @override
  _GridPhotoViewerState createState() => _GridPhotoViewerState();
}

class _GridTitleText extends StatefulWidget {
  const _GridTitleText(this.text);

  final String text;

  @override
  _GridTitleTextState createState() => _GridTitleTextState();
}

class _GridTitleTextState extends State<_GridTitleText> {
  @override
  Widget build(BuildContext context) {
    return FittedBox(
      fit: BoxFit.scaleDown,
      alignment: Alignment.centerLeft,
      child: Text(widget.text),
    );
  }
}

class _GridPhotoViewerState extends State<GridPhotoViewer>
    with SingleTickerProviderStateMixin {
  AnimationController _controller;
  Animation<Offset> _flingAnimation;
  Offset _offset = Offset.zero;
  double _scale = 1.0;
  Offset _normalizedOffset;
  double _previousScale;

  @override
  void initState() {
    super.initState();
    _controller = AnimationController(vsync: this)
      ..addListener(_handleFlingAnimation);
  }

  @override
  void dispose() {
    _controller.dispose();
    super.dispose();
  }

  // The maximum offset value is 0,0. If the size of this renderer's box is w,h
  // then the minimum offset value is w - _scale * w, h - _scale * h.
  Offset _clampOffset(Offset offset) {
    final Size size = context.size;
    final Offset minOffset = Offset(size.width, size.height) * (1.0 - _scale);
    return Offset(
      offset.dx.clamp(minOffset.dx, 0.0).toDouble(),
      offset.dy.clamp(minOffset.dy, 0.0).toDouble(),
    );
  }

  void _handleFlingAnimation() {
    setState(() {
      _offset = _flingAnimation.value;
    });
  }

  void _handleOnScaleStart(ScaleStartDetails details) {
    setState(() {
      _previousScale = _scale;
      _normalizedOffset = (details.focalPoint - _offset) / _scale;
      // The fling animation stops if an input gesture starts.
      _controller.stop();
    });
  }

  void _handleOnScaleUpdate(ScaleUpdateDetails details) {
    setState(() {
      _scale = (_previousScale * details.scale).clamp(1.0, 4.0).toDouble();
      // Ensure that image location under the focal point stays in the same place despite scaling.
      _offset = _clampOffset(details.focalPoint - _normalizedOffset * _scale);
    });
  }

  void _handleOnScaleEnd(ScaleEndDetails details) {
    final double magnitude = details.velocity.pixelsPerSecond.distance;
    if (magnitude < _kMinFlingVelocity) {
      return;
    }
    final Offset direction = details.velocity.pixelsPerSecond / magnitude;
    final double distance = (Offset.zero & context.size).shortestSide;
    _flingAnimation = _controller.drive(Tween<Offset>(
      begin: _offset,
      end: _clampOffset(_offset + direction * distance),
    ));
    _controller
      ..value = 0.0
      ..fling(velocity: magnitude / 1000.0);
  }

  @override
  Widget build(BuildContext context) {
    return GestureDetector(
      onScaleStart: _handleOnScaleStart,
      onScaleUpdate: _handleOnScaleUpdate,
      onScaleEnd: _handleOnScaleEnd,
      child: ClipRect(
        child: Transform(
          transform: Matrix4.identity()
            ..translate(_offset.dx, _offset.dy)
            ..scale(_scale),
          child: Image.asset(
            widget.photo.file,
            fit: BoxFit.cover,
          ),
        ),
      ),
    );
  }
}

class Media {
  Media({
    this.file,
  });

  final String file;

  static List<Media> getMedia() {
    return <Media>[
      Media(file: ICON_ADMIN_256),
      Media(file: ICON_ARCADE_256),
      Media(file: ICON_BITFLOW_256),
      Media(file: ICON_BOOKS_256),
      Media(file: ICON_CALENDAR_256),
      Media(file: ICON_CONTACTS_256),
      Media(file: ICON_DRIVE_256),
      Media(file: ICON_GALLERY_256),
      Media(file: ICON_MUSIC_256),
      Media(file: ICON_NOTES_256),
    ];
  }
}
