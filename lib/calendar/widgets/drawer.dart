import 'package:flutter/material.dart';

class CalendarDrawer extends StatefulWidget {
  const CalendarDrawer({Key key}) : super(key: key);

  @override
  _CalendarDrawerState createState() => _CalendarDrawerState();
}

class _CalendarDrawerState extends State<CalendarDrawer> {
  @override
  Widget build(BuildContext context) {
    return Container(
      child: const Text('Settings calendar'),
    );
  }
}
