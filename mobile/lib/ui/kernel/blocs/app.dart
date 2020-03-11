import 'dart:async';

import 'package:bloom/ui/kernel/blocs/bloc_provider.dart';

class AppBloc extends BlocBase {
  Apps _currentApp = Apps.HOME;
  Apps get currentApp => _currentApp;

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
