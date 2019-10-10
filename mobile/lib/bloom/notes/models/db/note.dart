import 'dart:convert';

import 'package:bloom/bloom/kernel/blocs/app.dart';
import 'package:bloom/bloom/kernel/services/db.dart';
import 'package:bloom/bloom/notes/models/gui.dart';
import 'package:bloom/native/core_ffi.dart';
import 'package:bloom/native/messages/notes.dart';
import 'package:flutter/foundation.dart';
import 'package:flutter/material.dart';
import 'package:sqflite/sqflite.dart';

class Note {
  Note({
    this.id,
    this.title = '',
    this.body = '',
    this.color = Colors.white,
    this.createdAt,
    this.updatedAt,
    this.archivedAt,
    this.isPinned = false,
  }) {
    createdAt = DateTime.now();
    updatedAt = DateTime.now();
    // db = DB();
  }
  String id;
  String title;
  String body;
  DateTime createdAt;
  DateTime updatedAt;
  Color color;
  DateTime archivedAt;
  bool isPinned;

  Map<String, dynamic> toMap() {
    final Map<String, dynamic> data = <String, dynamic>{
      'id': id,
      'title': title,
      'body': body,
      'created_at': _dateToEpochMs(createdAt),
      'updated_at': _dateToEpochMs(updatedAt),
      'color': color.value,
      'archived_at': _dateToEpochMs(archivedAt),
      'is_pinned': isPinned ? 1 : 0,
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
      isPinned: data['is_pinned'] == 1,
    );
  }

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

  static Future<DBNote> create(String title, String body, Color color) async {
    // Get a reference to the database
    debugPrint('Note.create called');
    // final Database db = await appBloc.db.db;

    // final Note note = Note(title: title, body: body, color: color);
    // note.id = Uuid().v4();
    // note.createdAt = DateTime.now();
    // note.updatedAt = DateTime.now();

    final Map<String, dynamic> res = await compute(
        Note._nativeCall, NotesGuiCreateNote(title, body, color.value));
    final NotesGuiNoteCreated ret = NotesGuiNoteCreated.fromJson(res);

    // final Map<String, dynamic> data = note.toMap();
    // debugPrint('note: $data');
    // // Insert the Note into the correct table.
    // await db.insert(DB.notesTable, data);
    return ret.note;
  }

  Future<Note> update() async {
    debugPrint('Note.update called (id: $id)');
    final Database db = await appBloc.db.db;

    updatedAt = DateTime.now();

    await db.update(
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
    final Database db = await appBloc.db.db;

    await db.delete(
      DB.notesTable,
      // Use a `where` clause to delete a specific note.
      where: 'id = ?',
      // Pass the note's id as a whereArg to prevent SQL injection.
      whereArgs: <String>[id],
    );
    return this;
  }

  static Future<List<DBNote>> find() async {
    // Get a reference to the database.
    // debugPrint('Note.find called');
    // final Database db = await appBloc.db.db;

    // // Query the table for all The Notes.
    // final List<Map<String, dynamic>> results = await db.query(
    //   DB.notesTable,
    //   where: 'archived_at IS NULL',
    //   orderBy: 'is_pinned DESC, created_at ASC',
    // );
    // debugPrint('fetched: ${results.length} notes');

    // return results.map(Note.fromMap).toList();

    final Map<String, dynamic> res =
        await compute(Note._nativeCall, NotesGuiListNotes());
    final NotesGuiNotes resMsg = NotesGuiNotes.fromJson(res);

    return resMsg.notes;
  }

  static Map<String, dynamic> _nativeCall<T>(T message) {
    final String jsonPayload = jsonEncode(message);
    debugPrint('input: $jsonPayload');
    final String res = coreFfi.call(jsonPayload);
    debugPrint('output: $res');
    return jsonDecode(res);
  }

  static Future<List<DBNote>> findArchived() async {
    // Get a reference to the database.
    debugPrint('Note.findArchived called');
    // final Database db = await appBloc.db.db;

    // // Query the table for all The archived Notes.
    // final List<Map<String, dynamic>> results = await db.query(
    //   DB.notesTable,
    //   where: 'archived_at IS NOT NULL',
    //   orderBy: 'is_pinned DESC, created_at ASC',
    // );

    final Map<String, dynamic> res =
        await compute(Note._nativeCall, NotesGuiListNotes());
    final NotesGuiNotes resMsg = NotesGuiNotes.fromJson(res);
    final List<DBNote> results = resMsg.notes;

    debugPrint('fetched: ${results.length} notes');

    return results;
  }
}
