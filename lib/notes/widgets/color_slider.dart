import 'package:flutter/material.dart';

class ColorSlider extends StatefulWidget {
  const ColorSlider(
      {@required this.callBackColorTapped, @required this.noteColor});
  final void Function(Color) callBackColorTapped;
  final Color noteColor;

  @override
  _ColorSliderState createState() => _ColorSliderState();
}

class _ColorSliderState extends State<ColorSlider> {
  final List<Color> colors = <Color>[
    const Color(0xffffffff), // classic white
    const Color(0xfff28b81), // light pink
    const Color(0xfff7bd02), // yellow
    const Color(0xfffbf476), // light yellow
    const Color(0xffcdff90), // light green
    const Color(0xffa7feeb), // turquoise
    const Color(0xffcbf0f8), // light cyan
    const Color(0xffafcbfa), // light blue
    const Color(0xffd7aefc), // plum
    const Color(0xfffbcfe9), // misty rose
    const Color(0xffe6c9a9), // light brown
    const Color(0xffe9eaee) // light gray
  ];

  final Color borderColor = const Color(0xffd3d3d3);
  final Color foregroundColor = const Color(0xff595959);

  final Icon _check = Icon(Icons.check);

  Color noteColor;
  int indexOfCurrentColor;
  @override
  void initState() {
    super.initState();
    noteColor = widget.noteColor;
    indexOfCurrentColor = colors.indexOf(noteColor);
  }

  @override
  Widget build(BuildContext context) {
    return ListView(
      scrollDirection: Axis.horizontal,
      children: List<GestureDetector>.generate(colors.length, (int index) {
        return GestureDetector(
            onTap: () => _colorChangeTapped(index),
            child: Padding(
                padding: const EdgeInsets.only(left: 6, right: 6),
                child: Container(
                    child: CircleAvatar(
                      child: _checkOrNot(index),
                      foregroundColor: foregroundColor,
                      backgroundColor: colors[index],
                    ),
                    width: 38.0,
                    height: 38.0,
                    padding: const EdgeInsets.all(1.0), // border width
                    decoration: BoxDecoration(
                      color: borderColor, // border color
                      shape: BoxShape.circle,
                    ))));
      }),
    );
  }

  void _colorChangeTapped(int indexOfColor) {
    setState(() {
      noteColor = colors[indexOfColor];
      indexOfCurrentColor = indexOfColor;
      widget.callBackColorTapped(colors[indexOfColor]);
    });
  }

  Widget _checkOrNot(int index) {
    if (indexOfCurrentColor == index) {
      return _check;
    }
    return null;
  }
}
