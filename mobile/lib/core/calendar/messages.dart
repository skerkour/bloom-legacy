import 'package:bloom/core/calendar/models.dart';

class CalendarListEvents {
  CalendarListEvents(this.startAt, this.endAt);
  final DateTime startAt;
  final DateTime endAt;

  Map<String, dynamic> toJson() {
    final Map<String, dynamic> data = <String, dynamic>{
      'startAt': startAt.toUtc().toIso8601String(),
      'endAt': endAt.toUtc().toIso8601String(),
    };
    return data;
  }
}

class CalendarEvents {
  CalendarEvents({this.events});

  final List<Event> events;

  static CalendarEvents fromJson(Map<String, dynamic> json) {
    final List<dynamic> list = json['events'];
    final List<Event> events =
        list.map((dynamic i) => Event.fromJson(i)).toList();
    return CalendarEvents(events: events);
  }
}

class CalendarCreateEvent {
  CalendarCreateEvent(this.title, this.description, this.startAt, this.endAt);

  final String title;
  final String description;
  final DateTime startAt;
  final DateTime endAt;

  Map<String, dynamic> toJson() {
    final Map<String, dynamic> data = <String, dynamic>{
      'title': title,
      'description': description,
      'startAt': startAt.toUtc().toIso8601String(),
      'endAt': endAt.toUtc().toIso8601String(),
    };
    return data;
  }
}

class CalendarDeleteEventParams {
  CalendarDeleteEventParams(this.id);

  final String id;

  Map<String, dynamic> toJson() {
    final Map<String, dynamic> data = <String, dynamic>{
      'id': id,
    };
    return data;
  }
}
