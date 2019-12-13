import 'package:bloom/bloom/preferences/widgets/drawer.dart';
import 'package:flutter/material.dart';

class PreferencesThemeView extends StatefulWidget {
  const PreferencesThemeView();

  @override
  _PreferencesThemeState createState() => _PreferencesThemeState();
}

class _PreferencesThemeState extends State<PreferencesThemeView> {
  @override
  Widget build(BuildContext context) {
    return Scaffold(
      drawer: const PreferencesDrawer(),
      appBar: AppBar(
        title: const Text('Preferences'),
      ),
      body: const Center(child: Text('PreferencesTheme')),
    );
  }
}
