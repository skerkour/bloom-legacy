import 'package:flutter/material.dart';

class ContactsDrawer extends StatefulWidget {
  const ContactsDrawer({Key key}) : super(key: key);

  @override
  _ContactsDrawerState createState() => _ContactsDrawerState();
}

class _ContactsDrawerState extends State<ContactsDrawer> {
  @override
  Widget build(BuildContext context) {
    return Container(
      child: const Text('Settings contacts'),
    );
  }
}
