import 'dart:async';

import 'package:bloom/kernel/blocs/bloc_provider.dart';
import 'package:bloom/notes/models/db/note.dart';
import 'package:flutter/material.dart';

class NotesBloc extends BlocBase {
  NotesBloc() {
    getNotes();
  }

  final StreamController<List<Note>> _notesController =
      StreamController<List<Note>>.broadcast();
  Stream<List<Note>> get notesStream => _notesController.stream;

  @override
  void dispose() {
    _notesController.close();
  }

  Future<void> getNotes() async {
    _notesController.sink.add(await Note.find());
  }

  Future<Note> delete(Note note) async {
    final Note ret = await note.delete();
    await getNotes();
    return ret;
  }

  Future<Note> create(String title, String body, Color color) async {
    final Note ret = await Note.create(title, body, color);
    await getNotes();
    return ret;
  }

  Future<Note> update(Note note) async {
    final Note ret = await note.update();
    await getNotes();
    return ret;
  }

  Future<Note> archive(Note note) async {
    final Note ret = await note.archive();
    await getNotes();
    return ret;
  }

  Future<Note> unarchive(Note note) async {
    final Note ret = await note.unarchive();
    await getNotes();
    return ret;
  }
}

final NotesBloc notesBloc = NotesBloc();
