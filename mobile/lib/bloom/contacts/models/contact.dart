import 'dart:convert';
import 'package:bloom/bloom/kernel/blocs/app.dart';
import 'package:bloom/bloom/kernel/services/db.dart';
import 'package:flutter/material.dart';
import 'package:sqflite/sqflite.dart';
import 'package:uuid/uuid.dart';
import 'package:bloom/native/core_ffi.dart';
import 'package:bloom/bloom/contacts/messages.dart';
import 'package:flutter/foundation.dart';

class Contact {
  Contact({
    this.id,
    this.name = '',
    this.deviceId,
  });

  String id;
  String name;
  String deviceId;

  Map<String, dynamic> toJson() {
    final Map<String, dynamic> data = <String, dynamic>{
      'id': id,
      'name': name,
      'device_id': deviceId,
    };
    return data;
  }

  static Contact fromJson(Map<String, dynamic> data) {
    return Contact(
      id: data['id'],
      name: data['name'],
      deviceId: data['device_id'],
    );
  }

  @override
  String toString() {
    return toJson().toString();
  }

  static Future<Contact> create(String name, String deviceId) async {
    // Get a reference to the database
    debugPrint('Contact.create called');
    final Database db = await appBloc.db.db;

    final Contact contact = Contact(name: name ?? '', deviceId: deviceId);
    contact.id = Uuid().v4();

    final Map<String, dynamic> data = contact.toJson();
    debugPrint('contact: $data');
    // Insert the Contact into the correct table.
    await db.insert(DB.contactsTable, data);
    return contact;
  }

  Future<Contact> update() async {
    debugPrint('Contact.update called (id: $id)');
    final Database db = await appBloc.db.db;

    await db.update(
      DB.contactsTable,
      toJson(),
      where: 'id = ?',
      whereArgs: <String>[id],
    );
    return this;
  }

  Future<Contact> delete() async {
    // Get a reference to the database
    debugPrint('Contact.delete called (id: $id)');
    final Database db = await appBloc.db.db;

    await db.delete(
      DB.contactsTable,
      // Use a `where` clause to delete a specific contact.
      where: 'id = ?',
      // Pass the coontact's id as a whereArg to prevent SQL injection.
      whereArgs: <String>[id],
    );
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
    final Database db = await appBloc.db.db;

    // Query the table for all The Contacts.
    final List<Map<String, dynamic>> results = await db.query(
      DB.contactsTable,
      where: 'device_id IS NOT NULL',
    );
    debugPrint('fetched: ${results.length} contacts');

    return results
        .map(Contact.fromJson)
        .map((Contact contact) => contact.deviceId)
        .toList();
  }

  static Map<String, dynamic> _nativeCall<T>(T message) {
    final String jsonPayload = jsonEncode(message);
    debugPrint('input: $jsonPayload');
    final String res = coreFfi.call(jsonPayload);
    debugPrint('output: $res');
    return jsonDecode(res);
  }
}
