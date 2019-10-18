import 'models/event.dart';

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
