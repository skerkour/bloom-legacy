import 'package:bloom/kernel/widgets/drawer.dart';
import 'package:flutter/material.dart';

class CalendarView extends StatefulWidget {
  const CalendarView();

  @override
  _CalendarState createState() => _CalendarState();
}

class _CalendarState extends State<CalendarView> {
  @override
  Widget build(BuildContext context) {
    return Scaffold(
      drawer: const BlmDrawer(),
      appBar: AppBar(
        title: const Text('Calendar'),
      ),
      body: Center(child: const Text('Calendar')),
    );
  }
}
