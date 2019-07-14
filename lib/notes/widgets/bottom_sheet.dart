import 'package:flutter/material.dart';

import 'color_slider.dart';

enum moreOptions { delete, share, copy }

class MoreOptionsSheet extends StatefulWidget {
  final Color color;
  final DateTime updatedAt;
  final void Function(Color) onColorChanged;
  final void Function() onDeleted;

  final void Function(moreOptions) callBackOptionTapped;

  const MoreOptionsSheet(
      {Key key,
      this.color,
      this.updatedAt,
      this.onColorChanged,
      this.onDeleted,
      this.callBackOptionTapped})
      : super(key: key);

  @override
  _MoreOptionsSheetState createState() => _MoreOptionsSheetState();
}

class _MoreOptionsSheetState extends State<MoreOptionsSheet> {
  Color noteColor;

  @override
  void initState() {
    noteColor = widget.color;
  }

  @override
  Widget build(BuildContext context) {
    return Container(
      color: this.noteColor,
      child: new Wrap(
        children: <Widget>[
          new ListTile(
            leading: new Icon(Icons.delete),
            title: new Text('Delete permanently'),
            onTap: _onDeleted,
          ),
          new ListTile(
              leading: new Icon(Icons.content_copy),
              title: new Text('Duplicate'),
              onTap: () {
                Navigator.of(context).pop();
                widget.callBackOptionTapped(moreOptions.copy);
              }),
          new ListTile(
              leading: new Icon(Icons.share),
              title: new Text('Share'),
              onTap: () {
                Navigator.of(context).pop();
                widget.callBackOptionTapped(moreOptions.share);
              }),
          new Padding(
            padding: EdgeInsets.only(left: 10, right: 10),
            child: SizedBox(
              height: 44,
              width: MediaQuery.of(context).size.width,
              child: ColorSlider(
                callBackColorTapped: _changeColor,
                // call callBack from notePage here
                noteColor: noteColor, // take color from local variable
              ),
            ),
          ),
          new Row(
            crossAxisAlignment: CrossAxisAlignment.center,
            children: [
              SizedBox(
                height: 44,
                child: Center(child: Text(widget.updatedAt.toString())),
              )
            ],
            mainAxisAlignment: MainAxisAlignment.center,
          ),
          new ListTile()
        ],
      ),
    );
  }

  void _changeColor(Color color) {
    setState(() {
      noteColor = color;
      widget.onColorChanged(color);
    });
  }

  void _onDeleted() {
    widget.onDeleted();
  }
}
