import 'dart:convert';
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

  static Note fromJson(Map<String, dynamic> json) {
    final String archivedAt = json['archived_at'];

    return Note(
      id: json['id'],
      title: json['title'],
      body: json['body'],
      createdAt: DateTime.parse(json['created_at']).toUtc(),
      updatedAt: DateTime.parse(json['updated_at']).toUtc(),
      color: Color(json['color']),
      archivedAt:
          archivedAt == null ? null : DateTime.parse(archivedAt).toUtc(),
      isPinned: json['is_pinned'],
    );
  }

  Map<String, dynamic> toJson() {
    final Map<String, dynamic> data = <String, dynamic>{
      'id': id,
      'title': title,
      'body': body,
      'created_at': createdAt.toUtc().toIso8601String(),
      'updated_at': updatedAt.toUtc().toIso8601String(),
      'color': color.value,
      'archived_at':
          archivedAt == null ? null : archivedAt.toUtc().toIso8601String(),
      'is_pinned': isPinned,
    };
    return data;
  }

  static Future<Note> create(String title, String body, Color color) async {
    debugPrint('Note.create called');

    final Map<String, dynamic> res = await compute(
        Note._nativeCall, NotesGuiCreateNote(title, body, color.value));
    final NotesGuiNote ret = NotesGuiNote.fromJson(res);

    return ret.note;
  }

  Future<Note> update() async {
    debugPrint('Note.update called (id: $id)');

    final Map<String, dynamic> res =
        await compute(Note._nativeCall, NotesGuiUpdateNote(this));
    final NotesGuiNote ret = NotesGuiNote.fromJson(res);

    return ret.note;
  }

  Future<void> delete() async {
    debugPrint('Note.delete called (id: $id)');
    await compute(Note._nativeCall, NotesGuiDeleteNote(id));
  }

  static Future<List<Note>> find() async {
    debugPrint('Note.find called');

    final Map<String, dynamic> res =
        await compute(Note._nativeCall, NotesGuiListNotes());
    final NotesGuiNotes resMsg = NotesGuiNotes.fromJson(res);

    return resMsg.notes;
  }

  static Map<String, dynamic> _nativeCall<T>(T message) {
    final String jsonPayload = jsonEncode(message);
    debugPrint('input: $jsonPayload');
    final Map<String, dynamic> res = coreFfi.call(jsonPayload);
    debugPrint('output: $res');
    return res;
  }

  static Future<List<Note>> findArchived() async {
    debugPrint('Note.findArchived called');

    final Map<String, dynamic> res =
        await compute(Note._nativeCall, NotesGetArchive());
    final NotesGuiNotes resMsg = NotesGuiNotes.fromJson(res);
    final List<Note> results = resMsg.notes;

    debugPrint('fetched: ${results.length} notes');

    return results;
  }
}
