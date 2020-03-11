import 'package:bloom/core/contacts/models.dart';
import 'package:flutter/material.dart';

class ContactsContacts {
  ContactsContacts({this.contacts});

  final List<Contact> contacts;

  static ContactsContacts fromJson(Map<String, dynamic> json) {
    final List<dynamic> list = json['data']['contacts'];
    final List<Contact> contacts =
        list.map((dynamic i) => Contact.fromJson(i)).toList();
    return ContactsContacts(contacts: contacts);
  }
}

// TODO(z0mbie42): missing fields
class ContactsCreateContact {
  ContactsCreateContact({
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
      'firstName': firstName,
      'lastName': lastName,
      'notes': notes,
      'deviceId': deviceId,
      'addresses': <String>[],
      'birthday': birthday == null ? null : birthday.toUtc().toIso8601String(),
      'organizations': <String>[],
      'emails': <String>[],
      'phones': <String>[],
      'websites': <String>[],
    };
    return data;
  }
}

class ContactsDeleteContact {
  ContactsDeleteContact(this.id);

  final String id;

  Map<String, dynamic> toJson() {
    final Map<String, dynamic> data = <String, dynamic>{
      'id': id,
    };
    return data;
  }
}
