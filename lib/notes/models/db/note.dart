import 'package:bloom/kernel/services/db.dart';
import 'package:flutter/material.dart';
import 'package:sqflite/sqflite.dart';
import 'package:uuid/uuid.dart';

class Note {
  Note(
      {this.id,
      this.title,
      this.body,
      this.color,
      this.createdAt,
      this.updatedAt,
      this.archivedAt});
  String id = '';
  String title = '';
  String body = '';
  DateTime createdAt = DateTime.now();
  DateTime updatedAt = DateTime.now();
  Color color = Colors.white;
  DateTime archivedAt = DateTime.now();

  Map<String, dynamic> toMap() {
    final Map<String, dynamic> data = <String, dynamic>{
      'id': id,
      'title': title,
      'body': body,
      'created_at': _dateToEpochMs(updatedAt),
      'updated_at': _dateToEpochMs(updatedAt),
      'color': color.value,
      'archived_at': _dateToEpochMs(archivedAt),
    };
    return data;
  }

// overriding toString() of the note class to print a better debug description of this custom class
  @override
  String toString() {
    return toMap().toString();
  }

  static int _dateToEpochMs(DateTime date) {
    if (date == null) {
      return null;
    } else {
      return date.millisecondsSinceEpoch;
    }
  }

  static DateTime _epochMsToDate(int epoch) {
    if (epoch == null) {
      return null;
    } else {
      return DateTime.fromMillisecondsSinceEpoch(epoch);
    }
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

  static Future<List<Note>> find() async {
    // Get a reference to the database.
    print('find called');
    final DB db = DB();
    final Database database = await db.db;

    // Query the table for all The Dogs.
    final List<Map<String, dynamic>> results = await database.query(DB.notesTable);
    //   DB.notesTable,
    //   where: 'archived_at = ?',
    //   whereArgs: <dynamic>[null],
    // );

    print('fetched: $results');

    return results.map((Map<String, dynamic> result) {
      print('result: $result');
      return Note(
        id: result['id'],
        title: result['title'],
        body: result['body'],
        archivedAt: _epochMsToDate(result['archived_at']),
        createdAt: _epochMsToDate(result['created_at']),
        updatedAt: _epochMsToDate(result['updated_at']),
        color: Color(result['color']),
      );
    }).toList();
  }
}
