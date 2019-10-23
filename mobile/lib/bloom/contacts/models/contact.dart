import 'dart:convert';
import 'package:bloom/bloom/kernel/blocs/app.dart';
import 'package:bloom/bloom/kernel/services/db.dart';
import 'package:flutter/material.dart';
import 'package:bloom/native/core_ffi.dart';
import 'package:bloom/bloom/contacts/messages.dart';
import 'package:flutter/foundation.dart';

class Contact {
  Contact({
    this.id,
    this.firstName = '',
    this.deviceId,
    this.createdAt,
    this.updatedAt,
    this.lastName,
    this.notes,
    this.birthday,
  });

  String id;
  String firstName;
  String deviceId;
  DateTime createdAt;
  DateTime updatedAt;
  String lastName;
  String notes;
  DateTime birthday;

  // TODO(z0mbie42): see below
  // pub addresses: serde_json::Value,
  // pub organizations: serde_json::Value,
  // pub emails: serde_json::Value,
  // pub phones: serde_json::Value,
  // pub websites: serde_json::Value,
  Map<String, dynamic> toJson() {
    final Map<String, dynamic> data = <String, dynamic>{
      'id': id,
      'device_id': deviceId,
      'created_at': createdAt.toUtc().toIso8601String(),
      'updated_at': updatedAt.toUtc().toIso8601String(),
      'first_name': firstName,
      'last_name': lastName,
      'notes': notes,
      'birthday': birthday ?? birthday.toUtc().toIso8601String(),
    };
    return data;
  }

  static Contact fromJson(Map<String, dynamic> data) {
    final String birthday = data['birthday'];
    return Contact(
      id: data['id'],
      firstName: data['first_name'],
      deviceId: data['device_id'],
      createdAt: DateTime.parse(data['created_at']).toUtc(),
      updatedAt: DateTime.parse(data['updated_at']).toUtc(),
      lastName: data['last_name'],
      notes: data['notes'],
      birthday: birthday ?? DateTime.parse(birthday).toUtc(),
    );
  }

  @override
  String toString() {
    return toJson().toString();
  }

  Future<Contact> create() async {
    debugPrint('Contact.create called');

    final Map<String, dynamic> res = await compute(
      Contact._nativeCall,
      ContactsGuiCreateContact(
        firstName: firstName,
        lastName: lastName,
        notes: notes,
        deviceId: deviceId,
        birthday: birthday,
      ),
    );
    final ContactsGuiContact ret = ContactsGuiContact.fromJson(res);

    return ret.contact;
  }

  Future<Contact> update() async {
    debugPrint('Contact.update called (id: $id)');

    final Map<String, dynamic> res =
        await compute(Contact._nativeCall, ContactsGuiUpdateContact(this));
    final ContactsGuiContact ret = ContactsGuiContact.fromJson(res);

    return ret.contact;
  }

  Future<Contact> delete() async {
    debugPrint('Contact.delete called (id: $id)');
    await compute(Contact._nativeCall, ContactsGuiDeleteContact(id));
    return this;
  }

  static Future<List<Contact>> list() async {
    debugPrint('Contact.find called');

    final Map<String, dynamic> res =
        await compute(Contact._nativeCall, ContactsGuiListContacts());
    final ContactsGuiContacts resMsg = ContactsGuiContacts.fromJson(res);

    return resMsg.contacts;
  }

  static Future<List<String>> findDeviceIds() async {
    // Get a reference to the database.
    debugPrint('Contact.find called');
    // final Database db = await appBloc.db.db;

    // // Query the table for all The Contacts.
    // final List<Map<String, dynamic>> results = await db.query(
    //   DB.contactsTable,
    //   where: 'device_id IS NOT NULL',
    // );
    // debugPrint('fetched: ${results.length} contacts');

    // return results
    //     .map(Contact.fromJson)
    //     .map((Contact contact) => contact.deviceId)
    //     .toList();
    return <String>[];
  }

  static Map<String, dynamic> _nativeCall<T>(T message) {
    final String jsonPayload = jsonEncode(message);
    debugPrint('input: $jsonPayload');
    final String res = coreFfi.call(jsonPayload);
    debugPrint('output: $res');
    return jsonDecode(res);
  }
}
