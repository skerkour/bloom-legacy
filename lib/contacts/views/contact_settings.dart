import 'package:flutter/material.dart';

class ContactSettings extends StatefulWidget {
  const ContactSettings({Key key}) : super(key: key);

  @override
  _ContactSettingsState createState() => _ContactSettingsState();
}

class _ContactSettingsState extends State<ContactSettings> {
  @override
  Widget build(BuildContext context) {
    return Container(
       child: const Text('Settings contacts'),
    );
  }
}