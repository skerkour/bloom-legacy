import 'package:bloom/core/contacts/messages.dart';
import 'package:bloom/core/contacts/methods.dart';
import 'package:bloom/core/core.dart';
import 'package:flutter/material.dart';
import 'package:flutter/foundation.dart';
import 'package:contacts_service/contacts_service.dart' as contacts_service;

class Contact {
  Contact({
    this.id,
    this.firstName = '',
    this.deviceId = '',
    this.createdAt,
    this.updatedAt,
    this.lastName = '',
    this.notes = '',
    this.birthday,
    this.emails = const <Email>[],
    this.phones = const <Phone>[],
    this.websites = const <Website>[],
    this.organizations = const <Organization>[],
  });

  String id;
  String firstName;
  String deviceId;
  DateTime createdAt;
  DateTime updatedAt;
  String lastName;
  String notes;
  DateTime birthday;
  List<Email> emails;
  List<Phone> phones;
  List<Website> websites;
  List<Organization> organizations;

  // TODO(z0mbie42): addresses
  Map<String, dynamic> toJson() {
    final Map<String, dynamic> data = <String, dynamic>{
      'id': id,
      'deviceId': deviceId,
      'createdAt': createdAt.toUtc().toIso8601String(),
      'updatedAt': updatedAt.toUtc().toIso8601String(),
      'firstName': firstName,
      'lastName': lastName,
      'notes': notes,
      'birthday': birthday == null ? null : birthday.toUtc().toIso8601String(),
      'addresses': <String>[],
      'organizations': organizations,
      'emails': emails,
      'phones': phones,
      'websites': websites,
    };
    return data;
  }

  static Contact fromJson(Map<String, dynamic> data) {
    final String birthday = data['birthday'];
    final List<dynamic> emails = data['emails'];
    final List<dynamic> phones = data['phones'];
    final List<dynamic> websites = data['websites'];
    final List<dynamic> organizations = data['organizations'];
    return Contact(
      id: data['id'],
      firstName: data['firstName'],
      deviceId: data['deviceId'],
      createdAt: DateTime.parse(data['createdAt']).toUtc(),
      updatedAt: DateTime.parse(data['updatedAt']).toUtc(),
      lastName: data['lastName'],
      notes: data['notes'],
      birthday: birthday != null ? DateTime.parse(birthday).toUtc() : null,
      emails: emails.map((dynamic i) => Email.fromJson(i)).toList(),
      phones: phones.map((dynamic i) => Phone.fromJson(i)).toList(),
      websites: websites.map((dynamic i) => Website.fromJson(i)).toList(),
      organizations:
          organizations.map((dynamic i) => Organization.fromJson(i)).toList(),
    );
  }

  static Contact fromDeviceContact(contacts_service.Contact contact) {
    return Contact(
      firstName: contact.givenName ?? '',
      lastName: contact.familyName ?? '',
      deviceId: contact.identifier ?? '',
    );
  }

  contacts_service.Contact toDeviceContact() {
    final contacts_service.Contact contact = contacts_service.Contact(
      givenName: firstName,
      familyName: lastName,
    );
    contact.identifier = deviceId;
    return contact;
  }

  @override
  String toString() {
    return toJson().toString();
  }

  Future<Contact> create() async {
    debugPrint('Contact.create called');

    final ContactsCreateContact params = ContactsCreateContact(
      firstName: firstName,
      lastName: lastName,
      notes: notes,
      deviceId: deviceId,
      birthday: birthday,
    );

    return Contact.fromJson(
        await coreCall(ContactsMethod.create_contact, params));
  }

  Future<Contact> update() async {
    debugPrint('Contact.update called (id: $id)');

    return Contact.fromJson(
        await coreCall(ContactsMethod.update_contact, this));
  }

  Future<Contact> delete() async {
    debugPrint('Contact.delete called (id: $id)');
    await coreCall(ContactsMethod.delete_contact, ContactsDeleteContact(id));
    return this;
  }

  static Future<List<Contact>> list() async {
    debugPrint('Contact.find called');
    final ContactsContacts resMsg = ContactsContacts.fromJson(
        await coreCall(ContactsMethod.list_contacts, Empty()));
    return resMsg.contacts;
  }

  static Future<Set<String>> findDeviceIds() async {
    // Get a reference to the database.
    debugPrint('Contact.find called');

    final List<Contact> contacts = await Contact.list();
    // ignore: prefer_collection_literals
    final Set<String> existingsContactsDeviceIds = Set<String>();
    for (Contact contact in contacts) {
      existingsContactsDeviceIds.add(contact.deviceId);
    }

    return existingsContactsDeviceIds;
  }
}

// export interface Organization {
//   name: string,
//   title: string,
// }

class Email {
  Email({
    @required this.email,
    @required this.label,
  });

  String email;
  String label;

  Map<String, dynamic> toJson() {
    final Map<String, dynamic> data = <String, dynamic>{
      'email': email,
      'label': label,
    };
    return data;
  }

  static Email fromJson(Map<String, dynamic> json) {
    return Email(email: json['email'], label: json['label']);
  }
}

class Phone {
  Phone({
    @required this.phone,
    @required this.label,
  });

  String phone;
  String label;

  Map<String, dynamic> toJson() {
    final Map<String, dynamic> data = <String, dynamic>{
      'phone': phone,
      'label': label,
    };
    return data;
  }

  static Phone fromJson(Map<String, dynamic> json) {
    return Phone(phone: json['phone'], label: json['label']);
  }
}

class Website {
  Website({
    @required this.website,
    @required this.label,
  });

  String website;
  String label;

  Map<String, dynamic> toJson() {
    final Map<String, dynamic> data = <String, dynamic>{
      'website': website,
      'label': label,
    };
    return data;
  }

  static Website fromJson(Map<String, dynamic> json) {
    return Website(website: json['website'], label: json['label']);
  }
}

class Organization {
  Organization({
    @required this.name,
    @required this.title,
  });

  String name;
  String title;

  Map<String, dynamic> toJson() {
    final Map<String, dynamic> data = <String, dynamic>{
      'name': name,
      'title': title,
    };
    return data;
  }

  static Organization fromJson(Map<String, dynamic> json) {
    return Organization(name: json['name'], title: json['title']);
  }
}
