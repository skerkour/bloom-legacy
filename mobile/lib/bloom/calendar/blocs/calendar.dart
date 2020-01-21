import 'dart:async';

import 'package:bloom/bloom/calendar/models/event.dart';
import 'package:bloom/bloom/kernel/blocs/bloc_provider.dart';

class CalendarBloc extends BlocBase {
  CalendarBloc() {
    _events = <DateTime, List<Event>>{};
    _selectedDay = _zeroizeDay(DateTime.now().toUtc()); // today
    selectedEvents = <Event>[];

    final DateTime nowUtc = DateTime.now().toUtc();
    _startDate = DateTime(nowUtc.year, nowUtc.month, 1);
    _endDate = _startDate.add(const Duration(days: 31));
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
  DateTime _startDate;
  DateTime _endDate;

  @override
  void dispose() {
    _eventsController.close();
  }

  Future<void> findEvents() async {
    final List<Event> events = await Event.find(_startDate, _endDate);
    _events = _aggregateEvents(events);

    _eventStream.add(_events);

    selectedEvents = _events[_selectedDay] ?? <Event>[];
    _selectedEventsStream.add(selectedEvents);
  }

  Future<void> updateSelectedDay(DateTime day) async {
    _selectedDay = _zeroizeDay(day);

    selectedEvents = _events[_selectedDay] ?? <Event>[];
    _selectedEventsStream.add(selectedEvents);
  }

  Future<void> updateStartAndEndDates(DateTime start, DateTime end) async {
    _startDate = start;
    _endDate = end;
    await findEvents();
  }

  Map<DateTime, List<Event>> _aggregateEvents(List<Event> events) {
    final Map<DateTime, List<Event>> ret = <DateTime, List<Event>>{};

    final int difference = _endDate.difference(_startDate).inDays + 1;

    for (Event event in events) {
      for (int i = 1; i <= difference; i += 1) {
        DateTime day = DateTime.utc(_startDate.year, _startDate.month, i);
        day = _zeroizeDay(day);
        if ((day.isAtSameMomentAs(event.startAt) ||
                event.startAt.isBefore(day)) &&
            (event.endAt.isAfter(day) || day.isAtSameMomentAs(event.endAt))) {
          List<Event> value = ret[day];
          if (value == null) {
            value = <Event>[event];
          } else {
            value.add(event);
          }
          ret[day] = value;
        }
      }
    }

    return ret;
  }

  DateTime _zeroizeDay(DateTime day) {
    return DateTime.parse(
        '${day.year}-${day.month.toString().padLeft(2, '0')}-${day.day.toString().padLeft(2, '0')}T00:00:00.00Z');
  }
}
