import 'package:bloom/ui/contacts/blocs/contact.dart';
import 'package:bloom/core/contacts/models.dart';
import 'package:bloom/ui/contacts/views/edit_contact.dart';
import 'package:flutter/material.dart';

enum ContactViewResult {
  Deleted,
}

enum PopupMenuAction {
  Deleted,
}

class ContactView extends StatefulWidget {
  const ContactView({@required this.contact});

  final Contact contact;

  @override
  _ContactState createState() => _ContactState();
}

class _ContactState extends State<ContactView> {
  ContactBloc _bloc;
  final TextEditingController _notesController = TextEditingController();

  @override
  void initState() {
    final Contact contact = widget.contact;
    _bloc = ContactBloc(contact: contact);
    _notesController.text = contact.notes;

    _bloc.deletedStream.listen((dynamic _) {
      Navigator.of(context).pop();
      Navigator.of(context).pop();
    });
    super.initState();
  }

  @override
  void dispose() {
    _bloc.dispose();
    _notesController.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
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
    final List<Widget> widgets = <Widget>[];

    widgets.add(ListTile(
      title: Text('${_bloc.contact.firstName} ${_bloc.contact.lastName}'),
      subtitle: Text(
        'Name',
        style: TextStyle(color: Colors.black54),
      ),
      leading: const Icon(Icons.person),
    ));

    // phone
    if (_bloc.contact.phones.isNotEmpty) {
      widgets.add(ListTile(
        title: Text(_bloc.contact.phones[0].phone),
        subtitle: Text(
          'Phone',
          style: TextStyle(color: Colors.black54),
        ),
        leading: Icon(Icons.phone),
        // leading: IconButton(
        //   icon: Icon(Icons.phone),
        //   onPressed: () {
        //     // _launchCaller(phoneNumber);
        //   },
        // ),
        // trailing: IconButton(
        //   icon: Icon(Icons.message),
        //   // onPressed: () {
        //   //   _textMe(phoneNumber);
        //   // },
        // ),
      ));
    }

    // emails
    if (_bloc.contact.emails.isNotEmpty) {
      widgets.add(ListTile(
        title: Text(_bloc.contact.emails[0].email),
        subtitle: Text(
          'Email',
          style: TextStyle(color: Colors.black54),
        ),
        leading: Icon(Icons.email),
      ));
    }

    if (_bloc.contact.notes.isNotEmpty) {
      widgets.add(Column(
        // mainAxisSize: MainAxisSize.min,
        children: <Widget>[
          ListTile(
            leading: Icon(Icons.note),
            title: Text('Notes', style: TextStyle(color: Colors.grey[600])),
          ),
          Padding(
            padding: const EdgeInsets.fromLTRB(15, 0, 15, 0),
            child: TextField(
              maxLines: 8,
              controller: _notesController,
              readOnly: true,
              decoration: const InputDecoration(
                border: InputBorder.none,
              ),
            ),
          ),
        ],
      ));
    }

    return SafeArea(
      child: Padding(
        padding: const EdgeInsets.all(8.0),
        child: Column(children: widgets),
      ),
    );
  }

  List<Widget> _buildAppBarActions(BuildContext context) {
    return <Widget>[
      IconButton(
        icon: Icon(
          Icons.edit,
          color: Colors.white,
        ),
        onPressed: () => _onEditPressed(context),
      ),
      PopupMenuButton<PopupMenuAction>(
        tooltip: 'Edit contact',
        itemBuilder: (BuildContext context) =>
            <PopupMenuEntry<PopupMenuAction>>[
          const PopupMenuItem<PopupMenuAction>(
            value: PopupMenuAction.Deleted,
            child: Text('Delete'),
          ),
        ],
        onSelected: (PopupMenuAction value) {
          switch (value) {
            case PopupMenuAction.Deleted:
              _onDeleted(context);
          }
        },
      ),
    ];
  }

  void _onDeleted(BuildContext context) {
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
      },
    );
  }

  void _onEditPressed(BuildContext context) {
    Navigator.push<dynamic>(
      context,
      MaterialPageRoute<dynamic>(
        builder: (BuildContext context) =>
            EditContactView(contact: _bloc.contact),
      ),
    );
  }
}

// class AddressesTile extends StatelessWidget {
//   const AddressesTile(this._addresses);
//   final Iterable<PostalAddress> _addresses;

//   @override
//   Widget build(BuildContext context) {
//     return Column(
//       crossAxisAlignment: CrossAxisAlignment.start,
//       children: <Widget>[
//         ListTile(title: const Text('Addresses')),
//         Column(
//           children: _addresses
//               .map((PostalAddress a) => Padding(
//                     padding: const EdgeInsets.symmetric(horizontal: 16.0),
//                     child: Column(
//                       children: <Widget>[
//                         ListTile(
//                           title: const Text('Street'),
//                           trailing: Text(a.street ?? ''),
//                         ),
//                         ListTile(
//                           title: const Text('Postcode'),
//                           trailing: Text(a.postcode ?? ''),
//                         ),
//                         ListTile(
//                           title: const Text('City'),
//                           trailing: Text(a.city ?? ''),
//                         ),
//                         ListTile(
//                           title: const Text('Region'),
//                           trailing: Text(a.region ?? ''),
//                         ),
//                         ListTile(
//                           title: const Text('Country'),
//                           trailing: Text(a.country ?? ''),
//                         ),
//                       ],
//                     ),
//                   ))
//               .toList(),
//         ),
//       ],
//     );
//   }
// }

// class ItemsTile extends StatelessWidget {
//   const ItemsTile(this._title, this._items);
//   final Iterable<Item> _items;
//   final String _title;

//   @override
//   Widget build(BuildContext context) {
//     return Column(
//       crossAxisAlignment: CrossAxisAlignment.start,
//       children: <Widget>[
//         ListTile(title: Text(_title)),
//         Column(
//           children: _items
//               .map(
//                 (Item i) => Padding(
//                   padding: const EdgeInsets.symmetric(horizontal: 16.0),
//                   child: ListTile(
//                     title: Text(i.label ?? ''),
//                     trailing: Text(i.value ?? ''),
//                   ),
//                 ),
//               )
//               .toList(),
//         ),
//       ],
//     );
//   }
// }
