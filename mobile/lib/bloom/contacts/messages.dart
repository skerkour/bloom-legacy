import 'package:flutter/material.dart';

import 'models/contact.dart';

class ContactsGuiListContacts {
  ContactsGuiListContacts();

  Map<String, dynamic> toJson() {
    final Map<String, dynamic> data = <String, dynamic>{
      'type': 'contacts.gui.list_contacts',
      'data': <String, dynamic>{},
    };
    return data;
  }
}

class ContactsGuiContacts {
  ContactsGuiContacts({this.contacts});

  final List<Contact> contacts;

  static ContactsGuiContacts fromJson(Map<String, dynamic> json) {
    final List<dynamic> list = json['data']['contacts'];
    final List<Contact> contacts =
        list.map((dynamic i) => Contact.fromJson(i)).toList();
    return ContactsGuiContacts(contacts: contacts);
  }
}

class ContactsGuiContact {
  ContactsGuiContact({this.contact});

  final Contact contact;

  static ContactsGuiContact fromJson(Map<String, dynamic> json) {
    final Contact contact = Contact.fromJson(json['data']['contact']);
    return ContactsGuiContact(contact: contact);
  }
}

// TODO(z0mbie42): missing fields
class ContactsGuiCreateContact {
  ContactsGuiCreateContact({
    @required this.firstName,
    @required this.lastName,
    @required this.notes,
    @required this.deviceId,
    @required this.birthday,
  });

  final String firstName;
  final String lastName;
  final String notes;
  final String deviceId;
  final DateTime birthday;

  Map<String, dynamic> toJson() {
    final Map<String, dynamic> data = <String, dynamic>{
      'type': 'contacts.gui.create_contact',
      'data': <String, dynamic>{
        'first_name': firstName,
        'last_name': lastName,
        'notes': notes,
        'device_id': deviceId,
        'addresses': <String>[],
        'birthday':
            birthday == null ? null : birthday.toUtc().toIso8601String(),
        'organizations': <String>[],
        'emails': <String>[],
        'phones': <String>[],
        'websites': <String>[],
      },
    };
    return data;
  }
}

class ContactsGuiUpdateContact {
  ContactsGuiUpdateContact(this.contact);

  final Contact contact;

  Map<String, dynamic> toJson() {
    final Map<String, dynamic> data = <String, dynamic>{
      'type': 'contacts.gui.update_contact',
      'data': <String, dynamic>{
        'contact': contact.toJson(),
      },
    };
    return data;
  }
}

class ContactsGuiDeleteContact {
  ContactsGuiDeleteContact(this.id);

  final String id;

  Map<String, dynamic> toJson() {
    final Map<String, dynamic> data = <String, dynamic>{
      'type': 'contacts.gui.delete_contact',
      'data': <String, dynamic>{
        'id': id,
      },
    };
    return data;
  }
}
