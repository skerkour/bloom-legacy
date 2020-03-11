import 'package:bloom/ui/kernel/services/utils.dart';
import 'package:bloom/core/notes/messages.dart';
import 'package:bloom/core/notes/methods.dart';
import 'package:bloom/core/core.dart';
import 'package:bloom/libs/hex_color.dart';
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
    final String archivedAt = json['archivedAt'];

    return Note(
      id: json['id'],
      title: json['title'],
      body: json['body'],
      createdAt: Utils.fromGoTime(json['createdAt']).toUtc(),
      updatedAt: Utils.fromGoTime(json['updatedAt']).toUtc(),
      color: HexColor.fromHex(json['color']),
      archivedAt:
          archivedAt == null ? null : Utils.fromGoTime(archivedAt).toUtc(),
      isPinned: json['isPinned'],
    );
  }

  Map<String, dynamic> toJson() {
    final Map<String, dynamic> data = <String, dynamic>{
      'id': id,
      'title': title,
      'body': body,
      'createdAt': createdAt.toUtc().toIso8601String(),
      'updatedAt': updatedAt.toUtc().toIso8601String(),
      'color': HexColor.toHex(color),
      'archivedAt':
          archivedAt == null ? null : archivedAt.toUtc().toIso8601String(),
      'isPinned': isPinned,
    };
    return data;
  }

  static Future<Note> create(String title, String body, Color color) async {
    debugPrint('Note.create called');

    return Note.fromJson(await coreCall(
        NotesMethod.create_note, NotesCreateNoteParams(title, body, color)));
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
