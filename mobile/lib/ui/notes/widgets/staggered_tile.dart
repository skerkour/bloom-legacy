import 'package:bloom/core/notes/models.dart';
import 'package:bloom/libs/auto_size_text/auto_size_text.dart';
import 'package:bloom/ui/notes/views/note.dart';
import 'package:flutter/material.dart';

class BlmStaggeredTile extends StatefulWidget {
  const BlmStaggeredTile({this.note});
  final Note note;

  @override
  _MyStaggeredTileState createState() => _MyStaggeredTileState();
}

class _MyStaggeredTileState extends State<BlmStaggeredTile> {
  String _content;
  double _fontSize;
  Color _tileColor;
  String _title;

  @override
  Widget build(BuildContext context) {
    _content = widget.note.body;
    _fontSize = _determineFontSizeForContent();
    _tileColor = widget.note.color;
    _title = widget.note.title;

    return GestureDetector(
      onTap: () => _noteTapped(context),
      child: Container(
        decoration: BoxDecoration(
            border: _tileColor == Colors.white
                ? Border.all(color: Colors.grey)
                : null,
            color: _tileColor,
            borderRadius: const BorderRadius.all(Radius.circular(8))),
        padding: const EdgeInsets.all(8),
        child: constructChild(),
      ),
    );
  }

  Future<void> _noteTapped(BuildContext ctx) async {
    // Navigator.pushNamed(ctx, '/notes/note', arguments: widget.note);
    final NoteViewResult res = await Navigator.push<dynamic>(
      context,
      MaterialPageRoute<dynamic>(
        builder: (BuildContext context) => NoteView(note: widget.note),
      ),
    );

    if (res != null) {
      Scaffold.of(context)
        ..removeCurrentSnackBar()
        ..showSnackBar(
          SnackBar(content: Text('Note ${res.toString().split('.').last}')),
        );
    }
  }

  Widget constructChild() {
    final List<Widget> contentsOfTiles = <Widget>[];

    if (widget.note.title.isNotEmpty) {
      contentsOfTiles.add(
        AutoSizeText(
          _title,
          style: TextStyle(fontSize: _fontSize, fontWeight: FontWeight.bold),
          maxLines: widget.note.title.isEmpty ? 1 : 3,
          textScaleFactor: 1.5,
        ),
      );
      contentsOfTiles.add(
        Divider(
          color: Colors.transparent,
          height: 6,
        ),
      );
    }

    contentsOfTiles.add(AutoSizeText(
      _content,
      style: TextStyle(fontSize: _fontSize),
      maxLines: 10,
      textScaleFactor: 1.5,
    ));

    return Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        mainAxisAlignment: MainAxisAlignment.start,
        children: contentsOfTiles);
  }

  double _determineFontSizeForContent() {
    final int charCount = _content.length + widget.note.title.length;
    double fontSize = 20;
    if (charCount > 110) {
      fontSize = 12;
    } else if (charCount > 80) {
      fontSize = 14;
    } else if (charCount > 50) {
      fontSize = 16;
    } else if (charCount > 20) {
      fontSize = 18;
    }

    return fontSize;
  }
}
