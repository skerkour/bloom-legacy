import 'dart:convert';
import 'package:bloom/kernel/services/db.dart';
import 'package:flutter/material.dart';
import 'package:sqflite/sqflite.dart';
import 'package:uuid/uuid.dart';

class Note {
  Note({ this.title, this.body, this.color});
  String id = '';
  String title = '';
  String body = '';
  DateTime createdAt = DateTime.now();
  DateTime updatedAt = DateTime.now();
  Color color = Colors.white;
  int isArchived = 0;

  Map<String, dynamic> toMap(bool forUpdate) {
    final Map<String, dynamic> data = <String, dynamic>{
//      'id': id,  since id is auto incremented in the database we don't need to send it to the insert query.
      'title': utf8.encode(title),
      'body': utf8.encode(body),
      'created_at': epochFromDate(createdAt),
      'updated_at': epochFromDate(updatedAt),
      'color': color.toString(),
      'is_archived': isArchived //  for later use for integrating archiving
    };
    if (forUpdate) {
      data['id'] = id;
    }
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
    return <String, dynamic>{
      'id': id,
      'title': title,
      'body': body,
      'created_at': epochFromDate(createdAt),
      'updated_at': epochFromDate(updatedAt),
      'color': color.toString(),
      'is_archived': isArchived
    }.toString();
  }

  static Future<Note> create(String title, String body, Color color) async {
    // Get a reference to the database
    print('insert called');
    final DB db = DB();
    final Database database = await db.db;
    final Uuid uuid = Uuid();

    final Note note = Note(title: title, body: body, color: color);
    note.id = uuid.v4();
    note.createdAt = DateTime.now();
    note.updatedAt = DateTime.now();

    // Insert the Notes into the correct table.
    await database.insert(DB.notesTable, note.toMap(false));
    return note;
  }
}
