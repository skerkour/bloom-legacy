import 'package:bloom/bitflow/views/bitflow.dart';
import 'package:bloom/calendar/views/calendar.dart';
import 'package:bloom/contacts/views/contacts.dart';
import 'package:bloom/drive/views/drive.dart';
import 'package:bloom/home/views/home.dart';
import 'package:bloom/notes/views/notes.dart';
import 'package:bloom/phaser/views/phaser.dart';
import 'package:bloom/platform/views/platform.dart';
import 'package:flutter/material.dart';

import 'auth/views/auth.dart';
import 'kernel/widgets/route_observer.dart';

void main() => runApp(MyApp());

class MyApp extends StatelessWidget {
  // This widget is the root of your application.
  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'Bloom',
      theme: ThemeData(
        primarySwatch: Colors.blue,
      ),
      initialRoute: '/',
      routes: <String, WidgetBuilder>{
        '/': (BuildContext context) => const HomeView(),
        '/notes': (BuildContext context) => const NotesView(),
        '/contacts': (BuildContext context) => ContactsView(),
        '/calendar': (BuildContext context) => const CalendarView(),
        '/drive': (BuildContext context) => const DriveView(),
        '/platform': (BuildContext context) => const PlatformView(),
        '/phaser': (BuildContext context) => const PhaserView(),
        '/bitflow': (BuildContext context) => const BitflowView(),
        '/auth': (BuildContext context) => const AuthView(),
      },
      navigatorObservers: <NavigatorObserver>[BlmRouteObserver()],
    );
  }
}
