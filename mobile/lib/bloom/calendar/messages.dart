import 'models/event.dart';

class CalendarListEvents {
  CalendarListEvents(this.startAt, this.endAt);
  final DateTime startAt;
  final DateTime endAt;

  Map<String, dynamic> toJson() {
    final Map<String, dynamic> data = <String, dynamic>{
      'type': 'calendar.gui.list_events',
      'data': <String, dynamic>{
        'start_at': startAt.toUtc().toIso8601String(),
        'end_at': endAt.toUtc().toIso8601String(),
      },
    };
    return data;
  }
}

class CalendarGuiEvents {
  CalendarGuiEvents({this.events});

  final List<Event> events;

  static CalendarGuiEvents fromJson(Map<String, dynamic> json) {
    final List<dynamic> list = json['data']['events'];
    final List<Event> events =
        list.map((dynamic i) => Event.fromJson(i)).toList();
    return CalendarGuiEvents(events: events);
  }
}

class CalendarGuiCreateEvent {
  CalendarGuiCreateEvent(
      this.title, this.description, this.startAt, this.endAt);

  final String title;
  final String description;
  final DateTime startAt;
  final DateTime endAt;

  Map<String, dynamic> toJson() {
    final Map<String, dynamic> data = <String, dynamic>{
      'type': 'calendar.gui.create_event',
      'data': <String, dynamic>{
        'title': title,
        'description': description,
        'start_at': startAt.toUtc().toIso8601String(),
        'end_at': endAt.toUtc().toIso8601String(),
      },
    };
    return data;
  }
}

class CalendarGuiEvent {
  CalendarGuiEvent({this.event});

  final Event event;

  static CalendarGuiEvent fromJson(Map<String, dynamic> json) {
    final Event event = Event.fromJson(json['data']['event']);
    return CalendarGuiEvent(event: event);
  }
}

class CalendarGuiUpdateEvent {
  CalendarGuiUpdateEvent(this.event);

  final Event event;

  Map<String, dynamic> toJson() {
    final Map<String, dynamic> data = <String, dynamic>{
      'type': 'calendar.gui.update_event',
      'data': <String, dynamic>{
        'event': event.toJson(),
      },
    };
    return data;
  }
}

class CalendarGuiDeleteEvent {
  CalendarGuiDeleteEvent(this.id);

  final String id;

  Map<String, dynamic> toJson() {
    final Map<String, dynamic> data = <String, dynamic>{
      'type': 'calendar.gui.delete_event',
      'data': <String, dynamic>{
        'id': id,
      },
    };
    return data;
  }
}
