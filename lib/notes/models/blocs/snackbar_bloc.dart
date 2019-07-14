import 'dart:async';

import 'package:bloom/kernel/blocs/bloc_provider.dart';

enum NotesSnackbarMessage { Archived }

class NotesSnackbarBloc extends BlocBase {
  final StreamController<NotesSnackbarMessage> _appsController =
      StreamController<NotesSnackbarMessage>.broadcast();
  Sink<NotesSnackbarMessage> get _inApps => _appsController.sink;
  Stream<NotesSnackbarMessage> get outApps => _appsController.stream;

  @override
  void dispose() {
    _appsController.close();
  }

  void archived() {
    _inApps.add(NotesSnackbarMessage.Archived);
  }
}

NotesSnackbarBloc notesSnackbarBloc = NotesSnackbarBloc();

enum Apps { HOME, NOTES, CONTACTS }
