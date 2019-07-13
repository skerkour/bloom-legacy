import 'dart:convert';
import 'package:bloom/kernel/services/db.dart';
import 'package:flutter/material.dart';
import 'package:sqflite/sqflite.dart';
import 'package:uuid/uuid.dart';

class Note {
  Note({this.title, this.body, this.color});
  String id = '';
  String title = '';
  String body = '';
  DateTime createdAt = DateTime.now();
  DateTime updatedAt = DateTime.now();
  Color color = Colors.white;
  int isArchived = 0;

  Map<String, dynamic> toMap() {
    final Map<String, dynamic> data = <String, dynamic>{
      'id': id,
      'title': title,
      'body': body,
      'created_at': epochFromDate(createdAt),
      'updated_at': epochFromDate(updatedAt),
      'color': color.toString(),
      'is_archived': isArchived //  for later use for integrating archiving
    };
    return data;
  }

// Converting the date time object into int representing seconds passed after midnight 1st Jan, 1970 UTC
  int epochFromDate(DateTime dt) {
    return dt.millisecondsSinceEpoch ~/ 1000;
  }

  void archiveThisNote() {
    isArchived = 1;
  }

// overriding toString() of the note class to print a better debug description of this custom class
  @override
  String toString() {
    return toMap().toString();
  }

  static Future<Note> create(String title, String body, Color color) async {
    // Get a reference to the database
    print('insert called');
    final DB db = DB();
    final Database database = await db.db;

    final Note note = Note(title: title, body: body, color: color);
    note.id = Uuid().v4();
    note.createdAt = DateTime.now();
    note.updatedAt = DateTime.now();
    print(note.toMap());

    // Insert the Notes into the correct table.
    await database.insert(DB.notesTable, note.toMap());
    return note;
  }

  Future<Note> update() async {
    // Get a reference to the database
    print('update called');
    final DB db = DB();
    final Database database = await db.db;
    updatedAt = DateTime.now();

    // Insert the Notes into the correct table.
    await database.update(
      DB.notesTable,
      toMap(),
      where: 'id = ?',
      // Pass the Dog's id as a whereArg to prevent SQL injection.
      whereArgs: <String>[id],
    );
    return this;
  }
}
