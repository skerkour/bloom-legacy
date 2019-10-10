import 'package:bloom/bloom/notes/models/gui.dart';

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

class NotesGetArchive {
  NotesGetArchive();

  Map<String, dynamic> toJson() {
    final Map<String, dynamic> data = <String, dynamic>{
      'type': 'notes.gui.get_archive',
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

class NotesGuiCreateNote {
  NotesGuiCreateNote(this.title, this.body, this.color);

  final String title;
  final String body;
  final int color;

  Map<String, dynamic> toJson() {
    final Map<String, dynamic> data = <String, dynamic>{
      'type': 'notes.gui.create_note',
      'data': <String, dynamic>{
        'title': title,
        'body': body,
        'color': color,
      },
    };
    return data;
  }
}

class NotesGuiNoteCreated {
  NotesGuiNoteCreated({this.note});

  final DBNote note;

  static NotesGuiNoteCreated fromJson(Map<String, dynamic> json) {
    final DBNote note = DBNote.fromJson(json['data']['note']);
    return NotesGuiNoteCreated(note: note);
  }
}
