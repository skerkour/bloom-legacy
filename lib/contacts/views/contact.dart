import 'package:bloom/contacts/blocs/contact.dart';
import 'package:contacts_service/contacts_service.dart';
import 'package:flutter/material.dart';

class ContactView extends StatefulWidget {
  const ContactView({this.contact});

  final Contact contact;

  @override
  _ContactState createState() => _ContactState();
}

class _ContactState extends State<ContactView> {
  ContactBloc _bloc;

  @override
  void initState() {
    final Contact contact = widget.contact ?? Contact();
    _bloc = ContactBloc(contact: contact);

    _bloc.deleted.listen((_) {
      Navigator.of(context).pop();
      Navigator.of(context).pop();
    });
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
        title: Text(_bloc.contact.displayName),
        actions: _buildAppBarActions(context),
      ),
      body: StreamBuilder<Contact>(
        initialData: _bloc.contact,
        stream: _bloc.contactOut,
        builder: (BuildContext context, AsyncSnapshot<Contact> snapshot) {
          if (snapshot.hasData) {
            return _buildBody(snapshot.data);
          } else {
            return Center(child: const CircularProgressIndicator());
          }
        },
      ),
    );
  }

  Widget _buildBody(Contact contact) {
    return SafeArea(
      child: ListView(
        children: <Widget>[
          ListTile(
            title: const Text('Name'),
            trailing: Text(contact.givenName ?? ''),
          ),
          ListTile(
            title: const Text('Middle name'),
            trailing: Text(contact.middleName ?? ''),
          ),
          ListTile(
            title: const Text('Family name'),
            trailing: Text(contact.familyName ?? ''),
          ),
          ListTile(
            title: const Text('Prefix'),
            trailing: Text(contact.prefix ?? ''),
          ),
          ListTile(
            title: const Text('Suffix'),
            trailing: Text(contact.suffix ?? ''),
          ),
          ListTile(
            title: const Text('Company'),
            trailing: Text(contact.company ?? ''),
          ),
          ListTile(
            title: const Text('Job'),
            trailing: Text(contact.jobTitle ?? ''),
          ),
          ListTile(
            title: const Text('Note'),
            trailing: Text(contact.note ?? ''),
          ),
          AddressesTile(contact.postalAddresses),
          ItemsTile('Phones', contact.phones),
          ItemsTile('Emails', contact.emails)
        ],
      ),
    );
  }

  List<Widget> _buildAppBarActions(BuildContext context) {
    return <Widget>[
      IconButton(
        icon: Icon(
          Icons.delete,
          color: Colors.white,
        ),
        onPressed: () => _onDeleted(context),
      ),
    ];
  }

  Function _onDeleted(BuildContext context) {
    return () {
      showDialog<Widget>(
          context: context,
          builder: (BuildContext context) {
            return AlertDialog(
              title: const Text('Confirm ?'),
              content: const Text('This contact will be permanently deleted'),
              actions: <Widget>[
                FlatButton(
                  onPressed: () {
                    // close dialog
                    Navigator.of(context).pop();
                  },
                  child: const Text('No'),
                ),
                FlatButton(
                  onPressed: _bloc.delete,
                  child: const Text('Yes'),
                ),
              ],
            );
          });
    }();
  }
}

class AddressesTile extends StatelessWidget {
  const AddressesTile(this._addresses);
  final Iterable<PostalAddress> _addresses;

  @override
  Widget build(BuildContext context) {
    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: <Widget>[
        ListTile(title: const Text('Addresses')),
        Column(
          children: _addresses
              .map((PostalAddress a) => Padding(
                    padding: const EdgeInsets.symmetric(horizontal: 16.0),
                    child: Column(
                      children: <Widget>[
                        ListTile(
                          title: const Text('Street'),
                          trailing: Text(a.street ?? ''),
                        ),
                        ListTile(
                          title: const Text('Postcode'),
                          trailing: Text(a.postcode ?? ''),
                        ),
                        ListTile(
                          title: const Text('City'),
                          trailing: Text(a.city ?? ''),
                        ),
                        ListTile(
                          title: const Text('Region'),
                          trailing: Text(a.region ?? ''),
                        ),
                        ListTile(
                          title: const Text('Country'),
                          trailing: Text(a.country ?? ''),
                        ),
                      ],
                    ),
                  ))
              .toList(),
        ),
      ],
    );
  }
}

class ItemsTile extends StatelessWidget {
  const ItemsTile(this._title, this._items);
  final Iterable<Item> _items;
  final String _title;

  @override
  Widget build(BuildContext context) {
    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: <Widget>[
        ListTile(title: Text(_title)),
        Column(
          children: _items
              .map(
                (Item i) => Padding(
                  padding: const EdgeInsets.symmetric(horizontal: 16.0),
                  child: ListTile(
                    title: Text(i.label ?? ''),
                    trailing: Text(i.value ?? ''),
                  ),
                ),
              )
              .toList(),
        ),
      ],
    );
  }
}
