import 'package:bloom/kernel/widgets/drawer.dart';
import 'package:flutter/material.dart';

class SettingsView extends StatefulWidget {
  @override
  _SettingsState createState() => _SettingsState();
}

class _SettingsState extends State<SettingsView> {
  @override
  Widget build(BuildContext context) {
    return Scaffold(
      drawer: const BlmDrawer(),
      appBar: AppBar(
        title: const Text('Contacts Settings'),
      ),
      body: _buildBody(),
    );
  }

  Container _buildBody() {
    return Container();
  }
}
