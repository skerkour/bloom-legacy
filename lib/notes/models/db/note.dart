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

  static Note fromMap(Map<String, dynamic> data) {
    return Note(
      id: data['id'],
      title: data['title'],
      body: data['body'],
      archivedAt: _epochMsToDate(data['archived_at']),
      createdAt: _epochMsToDate(data['created_at']),
      updatedAt: _epochMsToDate(data['updated_at']),
      color: Color(data['color']),
    );
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
    debugPrint('Note.create called');
    final DB db = DB();
    final Database database = await db.db;

    final Note note = Note(title: title, body: body, color: color);
    note.id = Uuid().v4();
    note.createdAt = DateTime.now();
    note.updatedAt = DateTime.now();

    final Map<String, dynamic> data = note.toMap();
    debugPrint('note: $data');
    // Insert the Note into the correct table.
    await database.insert(DB.notesTable, data);
    return note;
  }

  Future<Note> update() async {
    // Get a reference to the database
    debugPrint('Note.update called (id: $id)');
    final DB db = DB();
    final Database database = await db.db;

    updatedAt = DateTime.now();

    // Insert the Notes into the correct table.
    await database.update(
      DB.notesTable,
      toMap(),
      where: 'id = ?',
      whereArgs: <String>[id],
    );
    return this;
  }

  Future<Note> archive() async {
    // Get a reference to the database
    debugPrint('Note.archive called (id: $id)');
    final DB db = DB();
    final Database database = await db.db;

    updatedAt = DateTime.now();
    archivedAt = DateTime.now();

    // Insert the Notes into the correct table.
    await database.update(
      DB.notesTable,
      toMap(),
      where: 'id = ?',
      whereArgs: <String>[id],
    );
    return this;
  }

  Future<Note> delete() async {
    // Get a reference to the database
    debugPrint('Note.delete called (id: $id)');
    final DB db = DB();
    final Database database = await db.db;

    await database.delete(
      DB.notesTable,
      // Use a `where` clause to delete a specific note.
      where: 'id = ?',
      // Pass the note's id as a whereArg to prevent SQL injection.
      whereArgs: <String>[id],
    );
    return this;
  }

  static Future<List<Note>> find() async {
    // Get a reference to the database.
    debugPrint('Note.find called');
    final DB db = DB();
    final Database database = await db.db;

    // Query the table for all The Notes.
    final List<Map<String, dynamic>> results = await database.query(
      DB.notesTable,
      where: 'archived_at IS NULL',
    );
    debugPrint('fetched: ${results.length} notes');

    return results.map(Note.fromMap).toList();
  }

  static Future<List<Note>> findArchived() async {
    // Get a reference to the database.
    debugPrint('Note.findArchived called');
    final DB db = DB();
    final Database database = await db.db;

    // Query the table for all The Notes.
    final List<Map<String, dynamic>> results = await database.query(
      DB.notesTable,
      where: 'archived_at IS NOT NULL',
    );

    debugPrint('fetched: ${results.length} notes');

    return results.map(Note.fromMap).toList();
  }
}
