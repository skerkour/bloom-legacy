import 'package:flutter/material.dart';

class HomeSettings extends StatefulWidget {
  const HomeSettings({Key key}) : super(key: key);

  @override
  _HomeSettingsState createState() => _HomeSettingsState();
}

class _HomeSettingsState extends State<HomeSettings> {
  @override
  Widget build(BuildContext context) {
    return Container(
       child: const Text('SETTINGS HOME'),
    );
  }
}