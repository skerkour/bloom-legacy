import 'dart:async';

import 'package:bloom/bloom/kernel/blocs/bloc_provider.dart';
import 'package:bloom/bloom/kernel/services/db.dart';

class AppBloc extends BlocBase {
  AppBloc() : _db = DB();
  Apps _currentApp = Apps.HOME;
  Apps get currentApp => _currentApp;
  final DB _db;
  DB get db => _db;

  final StreamController<Apps> _currentAppController =
      StreamController<Apps>.broadcast();
  Sink<Apps> get _currentAppStream => _currentAppController.sink;
  Stream<Apps> get currentAppStream => _currentAppController.stream;

  @override
  void dispose() {
    _currentAppController.close();
  }

  void setCurrentApp(Apps app) {
    _currentApp = app;
    _currentAppStream.add(_currentApp);
  }
}

AppBloc appBloc = AppBloc();

enum Apps {
  HOME,
  NOTES,
  CONTACTS,
  CALENDAR,
  DRIVE,
}
