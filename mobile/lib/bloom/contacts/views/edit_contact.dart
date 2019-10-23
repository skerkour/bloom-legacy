import 'package:bloom/bloom/contacts/blocs/edit_contact.dart';
import 'package:bloom/bloom/contacts/models/contact.dart';
import 'package:flutter/material.dart';

class EditContactView extends StatefulWidget {
  const EditContactView({this.contact});

  final Contact contact;

  @override
  _EditContactState createState() => _EditContactState();
}

class _EditContactState extends State<EditContactView> {
  EditContactBloc _bloc;
  final TextEditingController _firstNameController = TextEditingController();
  final TextEditingController _lastNameController = TextEditingController();

  @override
  void initState() {
    final Contact contact = widget.contact ?? Contact();
    _bloc = EditContactBloc(contact: contact);

    // TODO(z0mbie42): missing fields
    _firstNameController.text = contact.firstName;
    _lastNameController.text = contact.lastName;

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
      appBar: AppBar(
        // Here we take the value from the MyHomePage object that was created by
        // the App.build method, and use it to set our appbar title.
        title: Text(_bloc.contact.firstName),
        actions: _buildAppBarActions(context),
      ),
      body: StreamBuilder<Contact>(
        initialData: _bloc.contact,
        stream: _bloc.contactStream,
        builder: (BuildContext context, AsyncSnapshot<Contact> snapshot) {
          if (snapshot.hasData) {
            return _buildBody(snapshot.data);
          } else {
            return const Center(child: CircularProgressIndicator());
          }
        },
      ),
    );
  }

  Widget _buildBody(Contact contact) {
    return SafeArea(
      child: Padding(
        padding: const EdgeInsets.all(12.0),
        child: Column(
          children: <Widget>[
            TextField(
              decoration: const InputDecoration(labelText: 'First name'),
              controller: _firstNameController,
            ),
            TextField(
              decoration: const InputDecoration(labelText: 'Last name'),
              controller: _lastNameController,
            )
          ],
        ),
      ),
    );
  }

  List<Widget> _buildAppBarActions(BuildContext context) {
    return <Widget>[
      FlatButton(
        onPressed: _onSavePressed,
        child: const Text('save', style: TextStyle(color: Colors.white)),
      )
    ];
  }

  // TODO(z0mbie42): missing fields
  Future<void> _onSavePressed() async {
    await _bloc.save(
      firstName: _firstNameController.text,
      lastName: _lastNameController.text,
      notes: '',
      deviceId: '',
      birthday: null,
    );
    Navigator.of(context).pop();
  }
}
