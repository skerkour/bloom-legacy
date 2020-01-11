import 'package:bloom/bloom/notes/models/note.dart';

class NotesNotes {
  NotesNotes({this.notes});

  final List<Note> notes;

  static NotesNotes fromJson(Map<String, dynamic> json) {
    final List<dynamic> list = json['data']['notes'];
    final List<Note> notes = list.map((dynamic i) => Note.fromJson(i)).toList();
    return NotesNotes(notes: notes);
  }
}

class NotesCreateNote {
  NotesCreateNote(this.title, this.body, this.color);

  final String title;
  final String body;
  final int color;

  Map<String, dynamic> toJson() {
    final Map<String, dynamic> data = <String, dynamic>{
      'title': title,
      'body': body,
      'color': color,
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
