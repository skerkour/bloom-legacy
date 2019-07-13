import 'dart:async';
import 'package:sqflite/sqflite.dart';
import 'package:path/path.dart';
import 'package:sqflite/sqlite_api.dart';

class DB {
  final String databaseName = 'bloom.db';
  static String notesTable = 'notes';
  Database _database;

  Future<Database> get db async {
    if (_database != null) {
      return _database;
    }

    _database = await init();
    return _database;
  }


  Future<Database> init() async {
    final String path = await getDatabasesPath();
    final String dbPath = join(path, databaseName);
    // ignore: argument_type_not_assignable
    final Database dbConnection = await openDatabase(
        dbPath, version: 1, onCreate: (Database db, int version) async {
      print('executing create query from onCreate callback');
      final String createNotestableQuery = '''
        CREATE TABLE IF NOT EXISTS $notesTable (
          id TEXT PRIMARY KEY,
          title TEXT,
          body TEXT,
          created_at INTEGER,
          updated_at INTEGER,
          color INTEGER,
          is_archived INTEGER
        )
      ''';
      await db.execute(createNotestableQuery);
    });

    // await dbConnection.execute(_buildCreateQuery());
    // _buildCreateQuery();
    return dbConnection;
  }

  // Future<int> insertNote(Note note, bool isNew) async {
  //   // Get a reference to the database
  //   final Database db = await database;
  //   print('insert called');

  //   // Insert the Notes into the correct table.
  //   await db.insert('notes',
  //     isNew ? note.toMap(false) : note.toMap(true),
  //     conflictAlgorithm: ConflictAlgorithm.replace,
  //   );

  //   if (isNew) {
  //     // get latest note which isn't archived, limit by 1
  //     var one = await db.query('notes', orderBy: 'date_last_edited desc',
  //         where: 'is_archived = ?',
  //         whereArgs: [0],
  //         limit: 1);
  //     int latestId = one.first['id'] as int;
  //     return latestId;
  //   }
  //   return note.id;
  // }


  // Future<bool> copyNote(Note note) async {
  //   final Database db = await database;
  //   try {
  //     await db.insert('notes',note.toMap(false), conflictAlgorithm: ConflictAlgorithm.replace);
  //   } catch(Error) {
  //     print(Error);
  //     return false;
  //   }
  //   return true;
  // }


  // Future<bool> archiveNote(Note note) async {
  //   if (note.id != -1) {
  //     final Database db = await database;

  //     int idToUpdate = note.id;

  //     db.update('notes', note.toMap(true), where: 'id = ?',
  //         whereArgs: [idToUpdate]);
  //   }
  // }

  // Future<bool> deleteNote(Note note) async {
  //   if(note.id != -1) {
  //     final Database db = await database;
  //     try {
  //       await db.delete('notes',where: 'id = ?',whereArgs: [note.id]);
  //       return true;
  //     } catch (Error){
  //       print('Error deleting ${note.id}: ${Error.toString()}');
  //       return false;
  //     }
  //   }
  // }


  // Future<List<Map<String,dynamic>>> selectAllNotes() async {
  //   final Database db = await database;
  //   // query all the notes sorted by last edited
  //   var data = await db.query('notes', orderBy: 'date_last_edited desc',
  //       where: 'is_archived = ?',
  //       whereArgs: [0]);

  //   return data;

  // }
}

