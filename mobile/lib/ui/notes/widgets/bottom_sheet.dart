import 'package:bloom/ui/kernel/services/utils.dart';
import 'package:flutter/material.dart';

import 'color_slider.dart';

enum moreOptions { delete, share, copy }

class MoreOptionsSheet extends StatefulWidget {
  const MoreOptionsSheet({
    Key key,
    this.color,
    this.updatedAt,
    this.onColorChanged,
    this.onDeleted,
    this.onShared,
    this.callBackOptionTapped,
    this.onDuplicated,
  }) : super(key: key);
  final Color color;
  final DateTime updatedAt;
  final void Function(Color) onColorChanged;
  final void Function() onDeleted;
  final void Function() onShared;
  final void Function() onDuplicated;

  final void Function(moreOptions) callBackOptionTapped;

  @override
  _MoreOptionsSheetState createState() => _MoreOptionsSheetState();
}

class _MoreOptionsSheetState extends State<MoreOptionsSheet> {
  Color noteColor;

  @override
  void initState() {
    noteColor = widget.color;
    super.initState();
  }

  @override
  Widget build(BuildContext context) {
    return Container(
      color: noteColor,
      child: Wrap(
        children: <Widget>[
          ListTile(
            leading: const Icon(Icons.delete),
            title: const Text('Delete permanently'),
            onTap: () => _onDeleted(context),
          ),
          ListTile(
            leading: Icon(Icons.content_copy),
            title: const Text('Duplicate'),
            onTap: () => _onDuplicated(context),
          ),
          ListTile(
            leading: Icon(Icons.share),
            title: const Text('Share'),
            onTap: () => _onShared(context),
          ),
          Padding(
            padding: const EdgeInsets.only(left: 10, right: 10),
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
          Row(
            crossAxisAlignment: CrossAxisAlignment.center,
            children: <Widget>[
              SizedBox(
                height: 44,
                child: Center(
                    child: Text(Utils.stringForDatetime(widget.updatedAt))),
              )
            ],
            mainAxisAlignment: MainAxisAlignment.center,
          ),
          const ListTile()
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

  void _onDeleted(BuildContext context) {
    Navigator.of(context).pop();
    widget.onDeleted();
  }

  void _onShared(BuildContext context) {
    Navigator.of(context).pop();
    widget.onShared();
  }

  void _onDuplicated(BuildContext context) {
    Navigator.of(context).pop();
    widget.onDuplicated();
  }
}
