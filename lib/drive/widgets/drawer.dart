import 'package:flutter/material.dart';

class DriveDrawer extends StatefulWidget {
  const DriveDrawer({Key key}) : super(key: key);

  @override
  _DriveDrawerState createState() => _DriveDrawerState();
}

class _DriveDrawerState extends State<DriveDrawer> {
  @override
  Widget build(BuildContext context) {
    return Container(
      child: const Text('Settings drive'),
    );
  }
}
