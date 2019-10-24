import 'dart:convert';

import 'package:bloom/native/core_ffi.dart';
import 'package:flutter/foundation.dart';
import 'package:flutter/material.dart';

import '../messages.dart';

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
      createdAt: DateTime.parse(json['created_at']).toUtc(),
      updatedAt: DateTime.parse(json['updated_at']).toUtc(),
      startAt: DateTime.parse(json['start_at']).toUtc(),
      endAt: DateTime.parse(json['end_at']).toUtc(),
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

    final Map<String, dynamic> res = await compute(Event._nativeCall,
        CalendarGuiCreateEvent(title, description, startAt, endAt));
    final CalendarGuiEvent ret = CalendarGuiEvent.fromJson(res);

    return ret.event;
  }

  Future<Event> update() async {
    debugPrint('Event.update called (id: $id)');

    final Map<String, dynamic> res =
        await compute(Event._nativeCall, CalendarGuiUpdateEvent(this));
    final CalendarGuiEvent ret = CalendarGuiEvent.fromJson(res);

    return ret.event;
  }

  Future<void> delete() async {
    debugPrint('Event.delete called (id: $id)');

    await compute(Event._nativeCall, CalendarGuiDeleteEvent(id));
  }

  static Future<List<Event>> find(DateTime startAt, DateTime endAt) async {
    debugPrint('Event.find called');

    final Map<String, dynamic> res =
        await compute(Event._nativeCall, CalendarListEvents(startAt, endAt));
    final CalendarGuiEvents resMsg = CalendarGuiEvents.fromJson(res);

    return resMsg.events;
  }

  static Map<String, dynamic> _nativeCall<T>(T message) {
    final String jsonPayload = jsonEncode(message);
    debugPrint('input: $jsonPayload');
    final Map<String, dynamic> res = coreFfi.call(jsonPayload);
    debugPrint('output: $res');
    return res;
  }

  DateTime _zeroizeDay(DateTime day) {
    return DateTime.parse('${day.year}-${day.month}-${day.day}T00:00:00.00Z');
  }
}
