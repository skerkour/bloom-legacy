import 'dart:convert';
import 'package:bloom/bloom/notes/models/gui.dart';
import 'package:bloom/native/core_ffi.dart';
import 'package:bloom/bloom/notes/messages.dart';
import 'package:flutter/foundation.dart';
import 'package:flutter/material.dart';

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
  });

  String id;
  String title;
  String body;
  DateTime createdAt;
  DateTime updatedAt;
  Color color;
  DateTime archivedAt;
  bool isPinned;

  static Future<DBNote> create(String title, String body, Color color) async {
    debugPrint('Note.create called');

    final Map<String, dynamic> res = await compute(
        Note._nativeCall, NotesGuiCreateNote(title, body, color.value));
    final NotesGuiNote ret = NotesGuiNote.fromJson(res);

    return ret.note;
  }

  // TODO(z): non static
  static Future<DBNote> update(DBNote note) async {
    debugPrint('Note.update called (id: ${note.id})');

    final Map<String, dynamic> res =
        await compute(Note._nativeCall, NotesGuiUpdateNote(note));
    final NotesGuiNote ret = NotesGuiNote.fromJson(res);

    return ret.note;
  }

  // TODO(z): non static
  static Future<void> delete(String noteId) async {
    debugPrint('Note.delete called (id: $noteId)');

    await compute(Note._nativeCall, NotesGuiDeleteNote(noteId));
  }

  static Future<List<DBNote>> find() async {
    debugPrint('Note.find called');

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
    debugPrint('Note.findArchived called');

    final Map<String, dynamic> res =
        await compute(Note._nativeCall, NotesGetArchive());
    final NotesGuiNotes resMsg = NotesGuiNotes.fromJson(res);
    final List<DBNote> results = resMsg.notes;

    debugPrint('fetched: ${results.length} notes');

    return results;
  }
}
