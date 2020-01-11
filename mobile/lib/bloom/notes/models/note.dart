import 'package:bloom/bloom/notes/core/messages.dart';
import 'package:bloom/bloom/notes/core/methods.dart';
import 'package:bloom/core.dart';
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
      Note._coreCall,
      core.toPayload(
          NotesMethod.create_note, NotesCreateNote(title, body, color.value)),
    );

    return Note.fromJson(res);
  }

  Future<Note> update() async {
    debugPrint('Note.update called (id: $id)');

    final Map<String, dynamic> res = await compute(
      Note._coreCall,
      core.toPayload(NotesMethod.update_note, this),
    );

    return Note.fromJson(res);
  }

  Future<void> delete() async {
    debugPrint('Note.delete called (id: $id)');

    await compute(
      Note._coreCall,
      core.toPayload(NotesMethod.delete_note, NotesDeleteNote(id)),
    );
  }

  static Future<List<Note>> find() async {
    debugPrint('Note.find called');

    final Map<String, dynamic> res = await compute(
      Note._coreCall,
      core.toPayload(NotesMethod.list_notes, Empty()),
    );
    final NotesNotes resMsg = NotesNotes.fromJson(res);

    return resMsg.notes;
  }

  static Map<String, dynamic> _coreCall(Payload<dynamic> payload) {
    final Map<String, dynamic> res = core.call(payload);
    debugPrint('output: $res');
    return res;
  }

  static Future<List<Note>> findArchived() async {
    debugPrint('Note.findArchived called');

    final Map<String, dynamic> res = await compute(
      Note._coreCall,
      core.toPayload(NotesMethod.list_archived, Empty()),
    );
    final NotesNotes resMsg = NotesNotes.fromJson(res);
    final List<Note> results = resMsg.notes;

    debugPrint('fetched: ${results.length} notes');

    return results;
  }
}
