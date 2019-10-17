import 'package:flutter/material.dart';
import 'mycolor.dart';

class Tile extends StatefulWidget {
  const Tile(this.number, this.width, this.height, this.color, this.size);

  final String number;
  final double width, height;
  final int color;
  final double size;

  @override
  State<StatefulWidget> createState() {
    // TODO(x): implement createState
    return _TileState();
  }
}

class _TileState extends State<Tile> {
  @override
  Widget build(BuildContext context) {
    // TODO(x): implement build
    return Container(
      child: Center(
        child: Text(
          widget.number,
          style: TextStyle(
              fontSize: widget.size,
              fontWeight: FontWeight.bold,
              color: Color(MyColor.fontColorTwoFour)),
        ),
      ),
      width: widget.width,
      height: widget.height,
      decoration: BoxDecoration(
          color: Color(widget.color),
          borderRadius: const BorderRadius.all(Radius.circular(10.0))),
    );
  }
}
