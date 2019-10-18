import 'dart:async';

import 'package:bloom/bloom/calendar/models/event.dart';
import 'package:bloom/bloom/kernel/blocs/bloc_provider.dart';

class CalendarBloc extends BlocBase {
  CalendarBloc();

  final StreamController<List<Event>> _eventsController =
      StreamController<List<Event>>.broadcast();
  StreamSink<List<Event>> get _eventStream => _eventsController.sink;
  Stream<List<Event>> get eventStream => _eventsController.stream;

  @override
  void dispose() {
    _eventsController.close();
  }

  Future<void> findEvents(DateTime startAt, DateTime endAt) async {
    _eventStream.add(await Event.find(startAt, endAt));
  }
}

final CalendarBloc notesBloc = CalendarBloc();
