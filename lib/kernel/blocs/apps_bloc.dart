import 'dart:async';

import 'package:bloom/kernel/blocs/bloc_provider.dart';

class AppsBloc extends BlocBase{

  Apps apps = Apps.HOME;

  final StreamController<Apps> _appsController =
  StreamController<Apps>.broadcast();
  Sink<Apps> get _inApps => _appsController.sink;
  Stream<Apps> get outApps => _appsController.stream;

  @override
  void dispose() {
    _appsController.close();
  }

  void setApps(Apps app) {
    apps = app;
    _inApps.add(apps);
  }
  
}

AppsBloc appsBloc = AppsBloc();

enum Apps {
  HOME,
  NOTES,
  CONTACTS
}