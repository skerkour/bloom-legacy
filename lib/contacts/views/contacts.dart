import 'package:bloom/kernel/widgets/drawer.dart';
import 'package:flutter/material.dart';

class Contacts extends StatefulWidget {
  @override
  _ContactsState createState() => _ContactsState();
}

class _ContactsState extends State<Contacts> {
  @override
  Widget build(BuildContext context) {
    return Scaffold(
      drawer: const BlmDrawer(),
      appBar: AppBar(
        // Here we take the value from the MyHomePage object that was created by
        // the App.build method, and use it to set our appbar title.
        title: const Text('Contacts'),
      ),
      body: Center(
        // Center is a layout widget. It takes a single child and positions it
        // in the middle of the parent.
        child: const Text('Contacts'),
      ),
      floatingActionButton: FloatingActionButton(
        onPressed: () => _newContactTapped(context),
        child: Icon(Icons.add),
      ),
    );
  }

  void _newContactTapped(BuildContext ctx) {
    print('new contact tapped');
  }
}
