import 'package:bloom/kernel/blocs/drawer_bloc.dart';
import 'package:flutter/material.dart';

class BlmRouteObserver extends RouteObserver<PageRoute<dynamic>> {
  BlmRouteObserver();

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
    if (routeStr == '/') {
      debugPrint('(RouteObserver) Home');
      drawerBloc.setApp(Apps.HOME);
    } else if (routeStr.startsWith('/notes')) {
      debugPrint('(RouteObserver) Notes');
      drawerBloc.setApp(Apps.NOTES);
    } else if (routeStr.startsWith('/contacts')) {
      drawerBloc.setApp(Apps.CONTACTS);
      debugPrint('(RouteObserver) Contacts');
    } else {
      debugPrint('(RouteObserver) route not found');
    }
  }
}
