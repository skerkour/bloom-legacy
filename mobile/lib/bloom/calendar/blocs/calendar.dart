import 'dart:async';

import 'package:bloom/bloom/calendar/models/event.dart';
import 'package:bloom/bloom/kernel/blocs/bloc_provider.dart';

class CalendarBloc extends BlocBase {
  CalendarBloc() {
    _events = <DateTime, List<Event>>{};
    final DateTime now = DateTime.now().toUtc();
    _selectedDay = DateTime.parse(
        '${now.year}-${now.month}-${now.day}T00:00:00.00Z'); // today
    selectedEvents = <Event>[];
  }

  final StreamController<Map<DateTime, List<Event>>> _eventsController =
      StreamController<Map<DateTime, List<Event>>>.broadcast();
  StreamSink<Map<DateTime, List<Event>>> get _eventStream =>
      _eventsController.sink;
  Stream<Map<DateTime, List<Event>>> get eventStream =>
      _eventsController.stream;

  final StreamController<List<Event>> _selectedEventsController =
      StreamController<List<Event>>.broadcast();
  StreamSink<List<Event>> get _selectedEventsStream =>
      _selectedEventsController.sink;
  Stream<List<Event>> get selectedEventsStream =>
      _selectedEventsController.stream;

  Map<DateTime, List<Event>> _events;
  DateTime _selectedDay;
  List<Event> selectedEvents;

  @override
  void dispose() {
    _eventsController.close();
  }

  Future<void> findEvents(DateTime startAt, DateTime endAt) async {
    final List<Event> events = await Event.find(startAt, endAt);
    _events = _aggregateEvents(events);

    _eventStream.add(_events);

    selectedEvents = _events[_selectedDay] ?? <Event>[];
    _selectedEventsStream.add(selectedEvents);
  }

  Future<void> updateSelectedEvents(List<Event> events) async {
    selectedEvents = events;
    _selectedEventsStream.add(events);
  }

  Map<DateTime, List<Event>> _aggregateEvents(List<Event> events) {
    final Map<DateTime, List<Event>> ret = <DateTime, List<Event>>{};

    for (Event event in events) {
      List<Event> value = ret[event.startAt];
      if (value == null) {
        value = <Event>[event];
      } else {
        value.add(event);
      }
      ret[event.startAt] = value;
    }
    return ret;
  }
}

final CalendarBloc notesBloc = CalendarBloc();
