import 'package:bloom/contacts/views/contacts.dart';
import 'package:bloom/kernel/views/home.dart';
import 'package:bloom/notes/views/archive.dart';
import 'package:bloom/notes/views/notes.dart';
import 'package:flutter/material.dart';

import 'kernel/widgets/route_observer.dart';
import 'notes/views/note.dart';

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
        '/': (BuildContext context) => const HomeView(title: 'Bloom'),
        '/notes': (BuildContext context) => NotesView(),
        '/notes/archive': (BuildContext context) => ArchiveView(),
        '/notes/note': (BuildContext context) => const NoteView(),
        '/contacts': (BuildContext context) => ContactsView(),
      },
      navigatorObservers: <NavigatorObserver>[BlmRouteObserver()],
    );
  }
}
