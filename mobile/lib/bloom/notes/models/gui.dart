import 'package:flutter/material.dart';

class DBNote {
  DBNote({
    this.id,
    this.title = '',
    this.body = '',
    this.color = Colors.white,
    this.createdAt,
    this.updatedAt,
    this.archivedAt,
    this.isPinned = false,
  }) {
    createdAt = DateTime.now();
    updatedAt = DateTime.now();
  }

  String id;
  String title;
  String body;
  DateTime createdAt;
  DateTime updatedAt;
  Color color;
  DateTime archivedAt;
  bool isPinned;

  static DBNote fromJson(Map<String, dynamic> json) {
    final String archivedAt = json['archived_at'];
    return DBNote(
      id: json['id'],
      title: json['title'],
      body: json['body'],
      createdAt: DateTime.parse(json['created_at']),
      updatedAt: DateTime.parse(json['updated_at']),
      color: Color(json['color']),
      archivedAt: archivedAt == null ? null : DateTime.parse(archivedAt),
      isPinned: json['is_pinned'],
    );
  }
}
