import 'package:bloom/kernel/blocs/apps_bloc.dart';
import 'package:flutter/material.dart';

class BlmRouteObserver extends RouteObserver<PageRoute<dynamic>> {
  BlmRouteObserver();

  @override
  void didPush(Route<dynamic> route, Route<dynamic> previousRoute) {
    super.didPush(route, previousRoute);
    debugPrint('didsPush');
    _onRouteChanged(route);
  }

  @override
  void didReplace({Route<dynamic> newRoute, Route<dynamic> oldRoute}) {
    super.didReplace(newRoute: newRoute, oldRoute: oldRoute);
    debugPrint('didReplace');
    _onRouteChanged(newRoute);
  }

  @override
  void didPop(Route<dynamic> route, Route<dynamic> previousRoute) {
    super.didPop(route, previousRoute);
    debugPrint('didPop');
    _onRouteChanged(previousRoute);
  }

  void _onRouteChanged(Route<dynamic> route) {
    final String routeStr = route.settings.name;
    if (routeStr == '/') {
      appsBloc.setApps(Apps.HOME);
    } else if (routeStr.startsWith('/notes')) {
      appsBloc.setApps(Apps.NOTES);
    } else if (routeStr.startsWith('/contacts')) {
    } else {
      debugPrint('(RouteObserver) route not found');
    }
  }
}
