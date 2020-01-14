import 'package:bloom/bloom/notes/core/messages.dart';
import 'package:bloom/bloom/notes/core/methods.dart';
import 'package:bloom/core.dart';
import 'package:bloom/libs/hex_color.dart';
import 'package:flutter/foundation.dart';
import 'package:flutter/material.dart';

class Note {
  Note({
    this.id,
    this.title = '',
    this.body = '',
    this.color,
    this.createdAt,
    this.updatedAt,
    this.archivedAt,
    this.isPinned = false,
  }) {
    color ??= HexColor.fromColor(Colors.white);
  }

  String id;
  String title;
  String body;
  DateTime createdAt;
  DateTime updatedAt;
  HexColor color;
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
      color: HexColor(json['color']),
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
      'color': color.toHex(),
      'archived_at':
          archivedAt == null ? null : archivedAt.toUtc().toIso8601String(),
      'is_pinned': isPinned,
    };
    return data;
  }

  static Future<Note> create(String title, String body, Color color) async {
    debugPrint('Note.create called');

    return Note.fromJson(await coreCall(
        NotesMethod.create_note, NotesCreateNote(title, body, color.value)));
  }

  Future<Note> update() async {
    debugPrint('Note.update called (id: $id)');

    return Note.fromJson(await coreCall(NotesMethod.update_note, this));
  }

  Future<void> delete() async {
    debugPrint('Note.delete called (id: $id)');

    await coreCall(NotesMethod.delete_note, NotesDeleteNote(id));
  }

  static Future<List<Note>> find() async {
    debugPrint('Note.find called');
    final NotesNotes resMsg =
        NotesNotes.fromJson(await coreCall(NotesMethod.list_notes, Empty()));
    return resMsg.notes;
  }

  static Future<List<Note>> findArchived() async {
    debugPrint('Note.findArchived called');

    final NotesNotes resMsg =
        NotesNotes.fromJson(await coreCall(NotesMethod.list_archived, Empty()));
    final List<Note> results = resMsg.notes;
    debugPrint('fetched: ${results.length} notes');

    return results;
  }
}
