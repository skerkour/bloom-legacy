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
  });

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
      createdAt: DateTime.parse(json['created_at']).toUtc(),
      updatedAt: DateTime.parse(json['updated_at']).toUtc(),
      color: Color(json['color']),
      archivedAt:
          archivedAt == null ? null : DateTime.parse(archivedAt).toUtc(),
      isPinned: json['is_pinned'],
    );
  }

  Map<String, dynamic> toJson() {
    final Map<String, dynamic> data = <String, dynamic>{
      'id': id,
      'title': title,
      'body': body,
      'created_at': createdAt.toUtc().toIso8601String(),
      'updated_at': updatedAt.toUtc().toIso8601String(),
      'color': color.value,
      'archived_at':
          archivedAt == null ? null : archivedAt.toUtc().toIso8601String(),
      'is_pinned': isPinned,
    };
    return data;
  }
}
