import 'dart:async';

import 'package:bloom/bloom/kernel/blocs/bloc_provider.dart';
import 'package:bloom/bloom/notes/models/db/note.dart';
import 'package:bloom/bloom/notes/models/gui.dart';

class NotesBloc extends BlocBase {
  NotesBloc();

  final StreamController<List<DBNote>> _notesController =
      StreamController<List<DBNote>>.broadcast();
  Stream<List<DBNote>> get notesOut => _notesController.stream;

  @override
  void dispose() {
    _notesController.close();
  }

  Future<void> getNotes() async {
    _notesController.sink.add(await Note.find());
  }

  Future<void> getArchive() async {
    _notesController.sink.add(await Note.findArchived());
  }
}

final NotesBloc notesBloc = NotesBloc();
