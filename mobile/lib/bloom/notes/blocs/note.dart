import 'dart:async';

import 'package:bloom/bloom/kernel/blocs/bloc_provider.dart';
import 'package:bloom/bloom/notes/models/db/note.dart';
import 'package:bloom/bloom/notes/models/gui.dart';
import 'package:flutter/material.dart';

class NoteBloc extends BlocBase {
  NoteBloc({@required DBNote note}) {
    _note = note;
  }

  DBNote _note;

  final StreamController<DBNote> _noteController =
      StreamController<DBNote>.broadcast();
  StreamSink<DBNote> get _noteStream => _noteController.sink;
  Stream<DBNote> get noteStream => _noteController.stream;

  final StreamController<DBNote> _noteDeletedController =
      StreamController<DBNote>.broadcast();
  StreamSink<DBNote> get _deletedStream => _noteDeletedController.sink;
  Stream<DBNote> get deletedStream => _noteDeletedController.stream;

  final StreamController<DBNote> _noteArchivedController =
      StreamController<DBNote>.broadcast();
  StreamSink<DBNote> get _archivedStream => _noteArchivedController.sink;
  Stream<DBNote> get archivedStream => _noteArchivedController.stream;

  final StreamController<DBNote> _noteUnarchivedController =
      StreamController<DBNote>.broadcast();
  StreamSink<DBNote> get _unarchivedStream => _noteUnarchivedController.sink;
  Stream<DBNote> get unarchivedStream => _noteUnarchivedController.stream;

  DBNote get note => _note;

  @override
  void dispose() {
    _noteController.close();
    _noteDeletedController.close();
    _noteArchivedController.close();
    _noteUnarchivedController.close();
  }

  Future<void> delete() async {
    if (_note.id != null) {
      await Note.delete(_note.id);
    }
    _deletedStream.add(_note);
  }

  Future<void> archive() async {
    _note.archivedAt = DateTime.now().toUtc();

    _note = await Note.update(_note);
    _noteStream.add(_note);
    _archivedStream.add(_note);
  }

  Future<void> unarchive() async {
    _note.archivedAt = null;

    _note = await Note.update(_note);
    _noteStream.add(_note);
    _unarchivedStream.add(_note);
  }

  Future<void> updateColor(Color color) async {
    _note.color = color;

    _note = await Note.update(_note);
    _noteStream.add(_note);
  }

  Future<void> pinUnpin() async {
    _note.isPinned = !_note.isPinned;

    _note = await Note.update(_note);
    _noteStream.add(_note);
  }

  Future<void> save(String title, String body) async {
    if (_note.title == title && _note.body == body) {
      return;
    }

    _note.title = title;
    _note.body = body;

    if (_note.id == null) {
      if (note.title.isEmpty && note.body.isEmpty) {
        debugPrint('note is empty, aborting');
        return;
      }
      _note = await Note.create(_note.title, _note.body, _note.color);
      debugPrint('note created');
    } else {
      _note = await Note.update(_note);
      debugPrint('note updated');
    }
    _noteStream.add(_note);
  }
}
