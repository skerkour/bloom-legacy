import 'dart:async';

import 'package:bloom/kernel/blocs/bloc_provider.dart';

class AppBloc extends BlocBase {
  Apps _currentApp = Apps.HOME;
  Apps get currentApp => _currentApp;

  final StreamController<Apps> _currentAppController =
      StreamController<Apps>.broadcast();
  Sink<Apps> get _inCurrentApp => _currentAppController.sink;
  Stream<Apps> get outCurrentApp => _currentAppController.stream;

  @override
  void dispose() {
    _currentAppController.close();
  }

  void setCurrentApp(Apps app) {
    _currentApp = app;
    _inCurrentApp.add(_currentApp);
  }
}

AppBloc appBloc = AppBloc();

enum Apps { HOME, NOTES, CONTACTS }
