import 'dart:async';
import 'package:bloom/core/calendar/models.dart';
import 'package:bloom/ui/kernel/blocs/bloc_provider.dart';
import 'package:flutter/material.dart';

class EditEventBloc extends BlocBase {
  EditEventBloc({@required Event event}) {
    _event = event;
  }

  Event _event;
  Event get event => _event;

  final StreamController<Event> _eventController =
      StreamController<Event>.broadcast();
  StreamSink<Event> get _eventStream => _eventController.sink;
  Stream<Event> get eventStream => _eventController.stream;

  @override
  void dispose() {
    _eventController.close();
  }

  Future<Event> save(String title, String description, DateTime startAt,
      DateTime endAt) async {
    debugPrint('EditEventBloc.save called');
    if (_event.title == title &&
        _event.description == description &&
        _event.startAt == startAt &&
        _event.endAt == endAt) {
      return _event;
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
        return _event;
      }
      _event = await Event.create(
          _event.title, _event.description, _event.startAt, _event.endAt);
      debugPrint('event created');
    } else {
      _event = await _event.update();
      debugPrint('event updated');
    }
    _eventStream.add(_event);
    return _event;
  }
}
