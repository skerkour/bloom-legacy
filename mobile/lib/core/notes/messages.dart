import 'dart:ui';

import 'package:bloom/core/notes/models.dart';
import 'package:bloom/libs/hex_color.dart';

class NotesNotes {
  NotesNotes({this.notes});

  final List<Note> notes;

  static NotesNotes fromJson(Map<String, dynamic> json) {
    final List<dynamic> list = json['notes'];
    final List<Note> notes = list.map((dynamic i) => Note.fromJson(i)).toList();
    return NotesNotes(notes: notes);
  }
}

class NotesCreateNoteParams {
  NotesCreateNoteParams(this.title, this.body, this.color);

  final String title;
  final String body;
  final Color color;

  Map<String, dynamic> toJson() {
    final Map<String, dynamic> data = <String, dynamic>{
      'title': title,
      'body': body,
      'color': HexColor.toHex(color),
    };
    return data;
  }
}

class NotesDeleteNote {
  NotesDeleteNote(this.id);

  final String id;

  Map<String, dynamic> toJson() {
    final Map<String, dynamic> data = <String, dynamic>{
      'id': id,
    };
    return data;
  }
}
