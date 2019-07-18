import 'dart:async';

import 'package:bloom/kernel/blocs/bloc_provider.dart';
import 'package:bloom/notes/models/db/note.dart';
import 'package:flutter/material.dart';

class NoteBloc extends BlocBase {
  NoteBloc({@required Note note}) {
    _note = note;
  }

  Note _note;

  final StreamController<Note> _noteController =
      StreamController<Note>.broadcast();
  StreamSink<Note> get _inNote => _noteController.sink;
  Stream<Note> get noteOut => _noteController.stream;

  final StreamController<Note> _noteDeletedController =
      StreamController<Note>.broadcast();
  StreamSink<Note> get _inDeleted => _noteDeletedController.sink;
  Stream<Note> get deleted => _noteDeletedController.stream;

  final StreamController<Note> _noteArchivedController =
      StreamController<Note>.broadcast();
  StreamSink<Note> get _inArchived => _noteArchivedController.sink;
  Stream<Note> get archived => _noteArchivedController.stream;

  final StreamController<Note> _noteUnarchivedController =
      StreamController<Note>.broadcast();
  StreamSink<Note> get _inUnarchived => _noteUnarchivedController.sink;
  Stream<Note> get unarchived => _noteUnarchivedController.stream;

  Note get note => _note;

  @override
  void dispose() {
    _noteController.close();
    _noteDeletedController.close();
    _noteArchivedController.close();
    _noteUnarchivedController.close();
  }

  Future<void> delete() async {
    if (_note.id != null) {
      _note = await _note.delete();
    }
    _inDeleted.add(_note);
  }

  Future<void> archive() async {
    _note.archivedAt = DateTime.now();

    _note = await _note.update();
    _inNote.add(_note);
    _inArchived.add(_note);
  }

  Future<void> unarchive() async {
    _note.archivedAt = null;

    _note = await _note.update();
    _inNote.add(_note);
    _inUnarchived.add(_note);
  }

  Future<void> updateColor(Color color) async {
    _note.color = color;

    _note = await _note.update();
    _inNote.add(_note);
  }

  Future<void> pinUnpin() async {
    _note.isPinned = !_note.isPinned;

    _note = await _note.update();
    _inNote.add(_note);
  }

  Future<void> save(String title, String body) async {
    if (_note.title == title && _note.body == body) {
      return;
    }

    _note.title = title;
    _note.body = body;

    if (_note.id == null) {
      // if (note.title.isEmpty && note.body.isEmpty) {
      //   debugPrint('note is empty, aborting');
      //   return;
      // }
      _note = await Note.create(_note.title, _note.body, _note.color);
      debugPrint('note created');
    } else {
      _note = await _note.update();
      debugPrint('note updated');
    }
    _inNote.add(_note);
  }
}
