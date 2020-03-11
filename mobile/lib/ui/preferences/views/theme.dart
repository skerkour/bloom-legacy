import 'package:bloom/ui/preferences/widgets/drawer.dart';
import 'package:flutter/material.dart';

class PreferencesThemeView extends StatefulWidget {
  const PreferencesThemeView();

  @override
  _PreferencesThemeState createState() => _PreferencesThemeState();
}

class _PreferencesThemeState extends State<PreferencesThemeView> {
  bool _darkMode = false;

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      drawer: const PreferencesDrawer(),
      appBar: AppBar(
        title: const Text('Preferences'),
      ),
      body: _buildBody(),
    );
  }

  Widget _buildBody() {
    return Container(
      child: SwitchListTile(
        title: const Text('Light theme'),
        value: _darkMode,
        onChanged: (bool value) {
          setState(() {
            _darkMode = value;
          });
        },
        secondary: const Icon(Icons.wb_sunny),
      ),
    );
  }
}
