import 'package:bloom/ui/kernel/blocs/app.dart';
import 'package:flutter/material.dart';

class BlmRouteObserver extends RouteObserver<PageRoute<dynamic>> {
  BlmRouteObserver();
  Apps _currentApp;

  @override
  void didPush(Route<dynamic> route, Route<dynamic> previousRoute) {
    super.didPush(route, previousRoute);
    debugPrint('didsPush');
    updateDrawer(route);
  }

  @override
  void didReplace({Route<dynamic> newRoute, Route<dynamic> oldRoute}) {
    super.didReplace(newRoute: newRoute, oldRoute: oldRoute);
    debugPrint('didReplace');
    updateDrawer(newRoute);
  }

  @override
  void didPop(Route<dynamic> route, Route<dynamic> previousRoute) {
    super.didPop(route, previousRoute);
    debugPrint('didPop');
    updateDrawer(previousRoute);
  }

  void updateDrawer(Route<dynamic> route) {
    final String routeStr = route.settings.name;
    if (routeStr == null) {
      return;
    }
    if (routeStr == '/') {
      debugPrint('(RouteObserver) Home');
      if (_currentApp != Apps.HOME) {
        _currentApp = Apps.HOME;
        appBloc.setCurrentApp(_currentApp);
      }
    } else if (routeStr.startsWith('/notes')) {
      debugPrint('(RouteObserver) Notes');
      if (_currentApp != Apps.NOTES) {
        _currentApp = Apps.NOTES;
        appBloc.setCurrentApp(_currentApp);
      }
    } else if (routeStr.startsWith('/contacts')) {
      debugPrint('(RouteObserver) Contacts');
      if (_currentApp != Apps.CONTACTS) {
        _currentApp = Apps.CONTACTS;
        appBloc.setCurrentApp(_currentApp);
      }
    } else if (routeStr.startsWith('/calendar')) {
      debugPrint('(RouteObserver) Calendar');
      if (_currentApp != Apps.CALENDAR) {
        _currentApp = Apps.CALENDAR;
        appBloc.setCurrentApp(_currentApp);
      }
    } else if (routeStr.startsWith('/drive')) {
      debugPrint('(RouteObserver) Drive');
      if (_currentApp != Apps.DRIVE) {
        _currentApp = Apps.DRIVE;
        appBloc.setCurrentApp(_currentApp);
      }
    } else {
      debugPrint('(RouteObserver) route not found');
    }
  }
}
