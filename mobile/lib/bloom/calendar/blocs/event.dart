import 'dart:async';
import 'package:bloom/bloom/calendar/models/event.dart';
import 'package:bloom/bloom/kernel/blocs/bloc_provider.dart';
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
    _eventController.close();
    _eventDeletedController.close();
  }

  Future<void> delete() async {
    debugPrint('EventBloc.delete called');
    if (_event.id != null) {
      await _event.delete();
    }
    _deletedStream.add(_event);
  }

  Future<void> save(String title, String description, DateTime startAt,
      DateTime endAt) async {
    debugPrint('EventBloc.save called');
    if (_event.title == title &&
        _event.description == description &&
        _event.startAt == startAt &&
        _event.endAt == endAt) {
      return;
    }

    _event.title = title;
    _event.description = description;
    _event.startAt = startAt;
    _event.endAt = endAt;

    if (_event.id == null) {
      if (_event.title.isEmpty &&
          _event.description.isEmpty &&
          _event.startAt == _event.endAt) {
        debugPrint('event is empty, aborting');
        return;
      }
      _event = await Event.create(
          _event.title, _event.description, _event.startAt, _event.endAt);
      debugPrint('event created');
    } else {
      _event = await _event.update();
      debugPrint('event updated');
    }
    _eventStream.add(_event);
  }
}
