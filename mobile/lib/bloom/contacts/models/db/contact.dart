import 'package:bloom/bloom/kernel/blocs/app.dart';
import 'package:bloom/bloom/kernel/services/db.dart';
import 'package:flutter/material.dart';
import 'package:sqflite/sqflite.dart';
import 'package:uuid/uuid.dart';

class Contact {
  //   Note({
  //   this.id,
  //   this.title = '',
  //   this.body = '',
  //   this.color = Colors.white,
  //   this.createdAt,
  //   this.updatedAt,
  //   this.archivedAt,
  //   this.isPinned = false,
  // }) {
  //   createdAt = DateTime.now();
  //   updatedAt = DateTime.now();
  //   // db = DB();
  // }
  // String id;
  // String title;
  // String body;
  // DateTime createdAt;
  // DateTime updatedAt;
  // Color color;
  // DateTime archivedAt;
  // bool isPinned;

  // Map<String, dynamic> toMap() {
  //   final Map<String, dynamic> data = <String, dynamic>{
  //     'id': id,
  //     'title': title,
  //     'body': body,
  //     'created_at': _dateToEpochMs(createdAt),
  //     'updated_at': _dateToEpochMs(updatedAt),
  //     'color': color.value,
  //     'archived_at': _dateToEpochMs(archivedAt),
  //     'is_pinned': isPinned ? 1 : 0,
  //   };
  //   return data;
  // }

  Contact({
    this.id,
    this.name = '',
    this.deviceId,
  });

  String id;
  String name;
  String deviceId;

  Map<String, dynamic> toMap() {
    final Map<String, dynamic> data = <String, dynamic>{
      'id': id,
      'name': name,
      'device_id': deviceId,
    };
    return data;
  }

  static Contact fromMap(Map<String, dynamic> data) {
    return Contact(
      id: data['id'],
      name: data['name'],
      deviceId: data['device_id'],
    );
  }

  @override
  String toString() {
    return toMap().toString();
  }

  static Future<Contact> create(String name, String deviceId) async {
    // Get a reference to the database
    debugPrint('Contact.create called');
    final Database db = await appBloc.db.db;

    final Contact contact = Contact(name: name ?? '', deviceId: deviceId);
    contact.id = Uuid().v4();

    final Map<String, dynamic> data = contact.toMap();
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
      toMap(),
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

  static Future<List<Contact>> find() async {
    // Get a reference to the database.
    debugPrint('Contact.find called');
    final Database db = await appBloc.db.db;

    // Query the table for all The Contacts.
    final List<Map<String, dynamic>> results = await db.query(
      DB.contactsTable,
    );
    debugPrint('fetched: ${results.length} contacts');

    return results.map(Contact.fromMap).toList();
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
        .map(Contact.fromMap)
        .map((Contact contact) => contact.deviceId)
        .toList();
  }
}
