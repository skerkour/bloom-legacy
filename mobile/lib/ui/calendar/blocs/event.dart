import 'dart:async';
import 'package:bloom/core/calendar/models.dart';
import 'package:bloom/ui/kernel/blocs/bloc_provider.dart';
import 'package:flutter/material.dart';

class EventBloc extends BlocBase {
  EventBloc({@required Event event}) {
    _event = event;
  }

  Event _event;

  final StreamController<Event> _eventController =
      StreamController<Event>.broadcast();
  StreamSink<Event> get _eventStream => _eventController.sink;
  Stream<Event> get eventStream => _eventController.stream;

  final StreamController<Event> _eventDeletedController =
      StreamController<Event>.broadcast();
  StreamSink<Event> get _deletedStream => _eventDeletedController.sink;
  Stream<Event> get deletedStream => _eventDeletedController.stream;

  Event get event => _event;

  @override
  void dispose() {
    _eventDeletedController.close();
    _eventController.close();
  }

  Future<void> delete() async {
    debugPrint('EventBloc.delete called');
    if (_event.id != null) {
      await _event.delete();
    }
    _deletedStream.add(_event);
  }

  void eventUpdated(Event event) {
    debugPrint('EVENT UPDATEDDDDD: ${event.toJson()}');
    _event = event;
    _eventStream.add(_event);
  }
}
