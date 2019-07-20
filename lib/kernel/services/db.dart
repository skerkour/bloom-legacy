import 'dart:async';
import 'package:sqflite/sqflite.dart';
import 'package:path/path.dart';
import 'package:sqflite/sqlite_api.dart';

class DB {
  final String databaseName = 'bloom.db';
  static String notesTable = 'notes';
  static Database _database;

  Future<Database> get db async {
    if (_database != null) {
      return _database;
    }

    _database = await _init();
    return _database;
  }

  Future<Database> _init() async {
    final String path = await getDatabasesPath();
    final String dbPath = join(path, databaseName);
    // ignore: argument_type_not_assignable
    final Database dbConnection = await openDatabase(dbPath, version: 1,
        onCreate: (Database db, int version) async {
      print('executing create query from onCreate callback');
      final String createNotestableQuery = '''
        CREATE TABLE IF NOT EXISTS $notesTable (
          id TEXT PRIMARY KEY NOT NULL,
          title TEXT NOT NULL,
          body TEXT NOT NULL,
          created_at INTEGER NOT NULL,
          updated_at INTEGER NOT NULL,
          color INTEGER NOT NULL,
          archived_at INTEGER,
          is_pinned INTEGER
        )
      ''';
      await db.execute(createNotestableQuery);
    });

    return dbConnection;
  }
}
