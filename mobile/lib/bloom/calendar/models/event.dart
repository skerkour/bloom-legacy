import 'package:bloom/bloom/calendar/core/messages.dart';
import 'package:bloom/bloom/calendar/core/methods.dart';
import 'package:bloom/bloom/kernel/services/utils.dart';
import 'package:bloom/core.dart';
import 'package:flutter/foundation.dart';
import 'package:flutter/material.dart';
import 'package:intl/intl.dart';

class Event {
  Event({
    this.id,
    this.title = '',
    this.description = '',
    this.createdAt,
    this.updatedAt,
    this.startAt,
    this.endAt,
  }) {
    final DateTime nowZero = _zeroizeDay(DateTime.now());
    startAt ??= nowZero;
    endAt ??= startAt;
  }

  String id;
  String title;
  String description;
  DateTime createdAt;
  DateTime updatedAt;
  DateTime startAt;
  DateTime endAt;

  static Event fromJson(Map<String, dynamic> json) {
    return Event(
      id: json['id'],
      title: json['title'],
      description: json['description'],
      createdAt: Utils.fromGoTime(json['created_at']).toUtc(),
      updatedAt: Utils.fromGoTime(json['updated_at']).toUtc(),
      startAt: Utils.fromGoTime(json['start_at']).toUtc(),
      endAt: Utils.fromGoTime(json['end_at']).toUtc(),
    );
  }

  Map<String, dynamic> toJson() {
    final Map<String, dynamic> data = <String, dynamic>{
      'id': id,
      'title': title,
      'description': description,
      'created_at': createdAt.toUtc().toIso8601String(),
      'updated_at': updatedAt.toUtc().toIso8601String(),
      'start_at': startAt.toUtc().toIso8601String(),
      'end_at': endAt.toUtc().toIso8601String(),
    };
    return data;
  }

  static Future<Event> create(String title, String description,
      DateTime startAt, DateTime endAt) async {
    debugPrint('Event.create called');

    return Event.fromJson(await coreCall(CalendarMethod.create_event,
        CalendarCreateEvent(title, description, startAt, endAt)));
  }

  Future<Event> update() async {
    debugPrint('Event.update called (id: $id)');
    return Event.fromJson(await coreCall(CalendarMethod.update_event, this));
  }

  Future<void> delete() async {
    debugPrint('Event.delete called (id: $id)');

    await coreCall(CalendarMethod.delete_event, CalendarDeleteEventParams(id));
  }

  static Future<List<Event>> find(DateTime startAt, DateTime endAt) async {
    debugPrint('Event.find called');
    final CalendarEvents resMsg = CalendarEvents.fromJson(await coreCall(
        CalendarMethod.list_events, CalendarListEvents(startAt, endAt)));

    return resMsg.events;
  }

  DateTime _zeroizeDay(DateTime day) {
    final DateFormat formatter = DateFormat('yyyy-MM-dd');
    return DateTime.parse('${formatter.format(day)}T00:00:00.00Z');
  }
}
