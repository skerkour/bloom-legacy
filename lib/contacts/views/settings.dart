import 'package:bloom/contacts/blocs/contacts.dart';
import 'package:bloom/contacts/widgets/drawer.dart';
import 'package:flutter/material.dart';

class SettingsView extends StatefulWidget {
  @override
  _SettingsState createState() => _SettingsState();
}

class _SettingsState extends State<SettingsView> {
    ContactsBloc _bloc;

  @override
  void initState() {
    _bloc = ContactsBloc();
    super.initState();
  }

  @override
  void dispose() {
    _bloc.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      endDrawer: const ContactsDrawer(),
      appBar: AppBar(
        title: const Text('Contacts Settings'),
      ),
      body: Builder(builder: (BuildContext context) {
          return _buildBody(context);
        }
      ),
    );
  }

  ListView _buildBody(BuildContext context) {
    return ListView(
          children: <Widget>[
            ListTile(
              leading: Icon(Icons.import_contacts),
              title: const Text('Import from device'),
              onTap: _onImportTapped(context),
            ),
          ],
        );
  }

  Function _onImportTapped(BuildContext context) {
    return () async {
    await _bloc.importContacts();
    Scaffold.of(context)
        ..removeCurrentSnackBar()
        ..showSnackBar(
          SnackBar(content: const Text('Contacts successfully imported')),
        );
    };
  }
}
