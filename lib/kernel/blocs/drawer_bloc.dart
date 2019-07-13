import 'dart:async';

import 'package:bloom/kernel/blocs/bloc_provider.dart';

class DrawerBloc extends BlocBase {
  Apps apps = Apps.HOME;

  final StreamController<Apps> _appsController =
      StreamController<Apps>.broadcast();
  Sink<Apps> get _inApps => _appsController.sink;
  Stream<Apps> get outApps => _appsController.stream;

  @override
  void dispose() {
    _appsController.close();
  }

  void setApp(Apps app) {
    apps = app;
    _inApps.add(apps);
  }
}

DrawerBloc drawerBloc = DrawerBloc();

enum Apps { HOME, NOTES, CONTACTS }
