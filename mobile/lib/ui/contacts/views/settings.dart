import 'package:bloom/ui/contacts/blocs/contacts.dart';
import 'package:bloom/ui/contacts/widgets/drawer.dart';
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
      }),
    );
  }

  ListView _buildBody(BuildContext context) {
    return ListView(
      children: <Widget>[
        ListTile(
          leading: Icon(Icons.phone_android),
          title: const Text('Import from device'),
          onTap: _onImportTapped(context),
        ),
      ],
    );
  }

  Function _onImportTapped(BuildContext context) {
    return () async {
      showDialog<dynamic>(
        context: context,
        barrierDismissible: false,
        builder: (BuildContext context) {
          return Dialog(
            child: Container(
              child: Padding(
                padding: const EdgeInsets.all(42.0),
                child: Row(
                  mainAxisSize: MainAxisSize.min,
                  children: const <Widget>[
                    CircularProgressIndicator(),
                    SizedBox(width: 20),
                    Text('Importing contacts'),
                  ],
                ),
              ),
            ),
          );
        },
      );
      final int imported = await _bloc.importContacts();
      Navigator.pop(context); //pop dialog
      Scaffold.of(context)
        ..removeCurrentSnackBar()
        ..showSnackBar(
          SnackBar(content: Text('$imported Contacts imported')),
        );
    };
  }
}
