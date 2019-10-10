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
    return DBNote(
      id: json['id'],
      title: json['title'],
      body: json['body'],
      createdAt: json['created_at'],
      updatedAt: json['updated_at'],
      color: json['color'],
      archivedAt: json['archived_at'],
      isPinned: json['is_pinned'],
    );
  }
}
