import 'package:bloom/notes/models/db/note.dart';
import 'package:bloom/notes/views/note.dart';
import 'package:flutter/material.dart';
import 'package:auto_size_text/auto_size_text.dart';
// import '../ViewControllers/NotePage.dart';
// import '../Models/Note.dart';
// import '../Models/Utility.dart';

class BlmStaggeredTile extends StatefulWidget {
  final Note note;
  BlmStaggeredTile(this.note);
  @override
  _MyStaggeredTileState createState() => _MyStaggeredTileState();
}



class _MyStaggeredTileState extends State<BlmStaggeredTile> {

  String _content ;
  double _fontSize ;
  Color tileColor ;
  String title;

  @override
  Widget build(BuildContext context) {

    _content = widget.note.body;
    _fontSize = _determineFontSizeForContent();
    tileColor = widget.note.color;
    title = widget.note.title;

    return GestureDetector(
      onTap: ()=> _noteTapped(context),
      child: Container(
      decoration: BoxDecoration(
        border: tileColor == Colors.white ?   Border.all(color: Colors.grey) : null,
          color: tileColor,
          borderRadius: BorderRadius.all(Radius.circular(8))),
      padding: EdgeInsets.all(8),
      child:  constructChild(),) ,
    );
  }

  void _noteTapped(BuildContext ctx) {
    Navigator.push<dynamic>(ctx, MaterialPageRoute<dynamic>(builder: (ctx) => NoteView(widget.note)));
  }

  Widget constructChild() {

    List<Widget> contentsOfTiles = [];

    if(widget.note.title.length != 0) {
      contentsOfTiles.add(
        AutoSizeText(
          title,
          style: TextStyle(fontSize: _fontSize,fontWeight: FontWeight.bold),
          maxLines: widget.note.title.length == 0 ? 1 : 3,
          textScaleFactor: 1.5,
        ),
      );
      contentsOfTiles.add(Divider(color: Colors.transparent,height: 6,),);
    }

    contentsOfTiles.add(
        AutoSizeText(
          _content,
          style: TextStyle(fontSize: _fontSize),
          maxLines: 10,
          textScaleFactor: 1.5,
        )
    );

    return Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        mainAxisAlignment: MainAxisAlignment.start,
        children:     contentsOfTiles
    );
  }


 double _determineFontSizeForContent() {
    int charCount = _content.length + widget.note.title.length ;
    double fontSize = 20 ;
    if (charCount > 110 ) { fontSize = 12; }
    else if (charCount > 80) {  fontSize = 14;  }
    else if (charCount > 50) {  fontSize = 16;  }
    else if (charCount > 20) {  fontSize = 18;  }


    return fontSize;
  }

}
