import 'package:flutter/material.dart';

class CalendarDrawer extends StatefulWidget {
  const CalendarDrawer({Key key}) : super(key: key);

  @override
  _CalendarDrawerState createState() => _CalendarDrawerState();
}

class _CalendarDrawerState extends State<CalendarDrawer> {
  @override
  Widget build(BuildContext context) {
    return Drawer(
      child: ListView(
        children: <Widget>[
          ListTile(
            leading: const Icon(Icons.date_range),
            title: const Text('Calendar'),
            onTap: () => debugPrint('Calendar tapped'),
          ),
        ],
      ),
    );
  }
}
