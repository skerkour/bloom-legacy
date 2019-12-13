import 'package:bloom/bloom/admin/views/dashboard.dart';
import 'package:bloom/bloom/arcade/views/arcade.dart';
import 'package:bloom/bloom/auth/views/registration_complete.dart';
import 'package:bloom/bloom/auth/views/registration_verify.dart';
import 'package:bloom/bloom/bitflow/views/bitflow.dart';
import 'package:bloom/bloom/books/views/books.dart';
import 'package:bloom/bloom/calculator/views/calculator.dart';
import 'package:bloom/bloom/calendar/views/calendar.dart';
import 'package:bloom/bloom/contacts/views/contacts.dart';
import 'package:bloom/bloom/drive/views/drive.dart';
import 'package:bloom/bloom/gallery/views/gallery.dart';
import 'package:bloom/bloom/home/views/home.dart';
import 'package:bloom/bloom/music/views/music.dart';
import 'package:bloom/bloom/notes/views/notes.dart';
import 'package:flutter/material.dart';

import 'package:bloom/bloom/auth/views/auth.dart';
import 'package:bloom/bloom/kernel/widgets/route_observer.dart';

import 'bloom/const.dart';
import 'bloom/phaser/views/scans.dart';

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
        '/admin': (BuildContext context) => const AdminDashboardView(),
        '/': (BuildContext context) => const HomeView(),
        '/arcade': (BuildContext context) => const ArcadeView(),
        '/auth': (BuildContext context) => const AuthView(),
        '/auth/registration/verify': (BuildContext context) =>
            const RegistrationVerifyView(),
        '/auth/registration/complete': (BuildContext context) =>
            const RegistrationCompleteView(),
        '/bitflow': (BuildContext context) => const BitflowView(),
        PATH_BOOKS: (BuildContext context) => const BooksView(),
        '/calculator': (BuildContext context) => const CalculatorView(),
        '/calendar': (BuildContext context) => const CalendarView(),
        '/contacts': (BuildContext context) => ContactsView(),
        PATH_DRIVE: (BuildContext context) => const DriveView(),
        PATH_GALLERY: (BuildContext context) => const GalleryView(),
        PATH_MUSIC: (BuildContext context) => const MusicView(),
        '/notes': (BuildContext context) => const NotesView(),
        PATH_PHASER: (BuildContext context) => const PhaserScansView(),
      },
      navigatorObservers: <NavigatorObserver>[BlmRouteObserver()],
    );
  }
}
