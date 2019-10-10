import 'package:bloom/bloom/notes/models/gui.dart';
import 'package:flutter/cupertino.dart';

class NotesGuiListNotes {
  NotesGuiListNotes();

  Map<String, dynamic> toJson() {
    final Map<String, dynamic> data = <String, dynamic>{
      'type': 'notes.gui.list_notes',
      'data': <String, dynamic>{},
    };
    return data;
  }
}

class NotesGuiNotes {
  NotesGuiNotes({this.notes});

  final List<DBNote> notes;

  static NotesGuiNotes fromJson(Map<String, dynamic> json) {
    final List<dynamic> list = json['data']['notes'];
    final List<DBNote> notes =
        list.map((dynamic i) => DBNote.fromJson(i)).toList();
    return NotesGuiNotes(notes: notes);
  }
}
