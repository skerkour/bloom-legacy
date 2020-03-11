import 'package:bloom/ui/contacts/views/contacts.dart';
import 'package:bloom/ui/contacts/views/settings.dart';
import 'package:flutter/material.dart';

class ContactsDrawer extends StatefulWidget {
  const ContactsDrawer({Key key}) : super(key: key);

  @override
  _ContactsDrawerState createState() => _ContactsDrawerState();
}

class _ContactsDrawerState extends State<ContactsDrawer> {
  @override
  Widget build(BuildContext context) {
    return Drawer(
      child: ListView(
        children: <Widget>[
          ListTile(
            leading: const Icon(Icons.list),
            title: const Text('Contacts'),
            onTap: () {
              Navigator.pop(context);
              Navigator.pushAndRemoveUntil<dynamic>(
                context,
                MaterialPageRoute<dynamic>(
                  builder: (BuildContext context) => ContactsView(),
                ),
                (Route<dynamic> route) => false,
              );
            },
          ),
          ListTile(
            leading: const Icon(Icons.settings),
            title: const Text('Settings'),
            onTap: () {
              Navigator.pop(context);
              Navigator.push<dynamic>(
                context,
                MaterialPageRoute<dynamic>(
                  builder: (BuildContext context) => SettingsView(),
                ),
              );
            },
          ),
        ],
      ),
    );
  }
}
