import 'package:bloom/bloom/admin/views/dashboard.dart';
import 'package:bloom/bloom/arcade/views/arcade.dart';
import 'package:bloom/bloom/auth/views/registration_complete.dart';
import 'package:bloom/bloom/auth/views/registration_verify.dart';
import 'package:bloom/bloom/bitflow/views/downloads.dart';
import 'package:bloom/bloom/calculator/views/calculator.dart';
import 'package:bloom/bloom/calendar/views/calendar.dart';
import 'package:bloom/bloom/contacts/views/contacts.dart';
import 'package:bloom/bloom/drive/views/drive.dart';
import 'package:bloom/bloom/home/views/home.dart';
import 'package:bloom/bloom/myaccount/views/profile.dart';
import 'package:bloom/bloom/notes/views/notes.dart';
import 'package:bloom/bloom/preferences/views/theme.dart';
import 'package:bloom/core.dart';
import 'package:flutter/material.dart';

import 'package:bloom/bloom/auth/views/auth.dart';
import 'package:bloom/bloom/kernel/widgets/route_observer.dart';

import 'bloom/const.dart';
import 'bloom/qrcodes/views/qrcodes.dart';

Future<void> main() async {
  await coreInit();
  runApp(MyApp());
}

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
        PATH_ADMIN: (BuildContext context) => const AdminDashboardView(),
        '/': (BuildContext context) => const HomeView(),
        PATH_ARCADE: (BuildContext context) => const ArcadeView(),
        '/auth': (BuildContext context) => const AuthView(),
        '/auth/registration/verify': (BuildContext context) =>
            const RegistrationVerifyView(),
        '/auth/registration/complete': (BuildContext context) =>
            const RegistrationCompleteView(),
        PATH_BITFLOW: (BuildContext context) => const BitflowDownloadsView(),
        PATH_CALCULATOR: (BuildContext context) => const CalculatorView(),
        PATH_CALENDAR: (BuildContext context) => const CalendarView(),
        PATH_CONTACTS: (BuildContext context) => ContactsView(),
        PATH_DRIVE: (BuildContext context) => const DriveView(),
        PATH_MYACCOUNT: (BuildContext context) => const MyAccountProfileView(),
        PATH_NOTES: (BuildContext context) => const NotesView(),
        PATH_PREFERENCES: (BuildContext context) =>
            const PreferencesThemeView(),
        PATH_QRCODES: (BuildContext context) => const QrCodesView(),
      },
      navigatorObservers: <NavigatorObserver>[BlmRouteObserver()],
    );
  }
}
