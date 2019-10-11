use bloom_error::BloomError;
use bloom_messages::{notes, Message};
use bloom_models::notes::db;
use rusqlite::{params, Connection, NO_PARAMS};

pub fn list_notes(_: notes::GuiListNotes) -> Result<Message, BloomError> {
    let contents = std::fs::read_to_string("/proc/self/cmdline")?;
    let contents = contents.trim_end_matches('\x00');
    let db_dir = format!("/data/data/{}/databases", &contents);
    std::fs::create_dir_all(&db_dir)?;
    let db_path = format!("{}/bloom42.db", &db_dir);
    let conn = Connection::open(&db_path)?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS notes (
            id TEXT PRIMARY KEY NOT NULL,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL,
            archived_at TEXT,
            title TEXT NOT NULL,
            body TEXT NOT NULL,
            color INTEGER NOT NULL,
            is_pinned INTEGER
        )",
        NO_PARAMS,
    )?;

    let mut stmt = conn.prepare(
        "SELECT * FROM notes WHERE archived_at IS NULL ORDER BY is_pinned DESC, created_at ASC",
    )?;
    let notes_iter = stmt.query_map(NO_PARAMS, |row| {
        Ok(db::Note {
            id: row.get(0)?,
            created_at: row.get(1)?,
            updated_at: row.get(2)?,
            archived_at: row.get(3)?,
            title: row.get(4)?,
            body: row.get(5)?,
            color: row.get(6)?,
            is_pinned: row.get(7)?,
        })
    })?;
    let notes_found: Result<Vec<db::Note>, _> = notes_iter.collect();
    let notes_found = notes_found?;

    let ret: Message = notes::GuiNotes { notes: notes_found }.into();

    return Ok(ret);
}

pub fn get_archive(_: notes::GuiGetArchive) -> Result<Message, BloomError> {
    let contents = std::fs::read_to_string("/proc/self/cmdline")?;
    let contents = contents.trim_end_matches('\x00');
    let db_dir = format!("/data/data/{}/databases", &contents);
    std::fs::create_dir_all(&db_dir)?;
    let db_path = format!("{}/bloom42.db", &db_dir);
    let conn = Connection::open(&db_path)?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS notes (
            id TEXT PRIMARY KEY NOT NULL,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL,
            archived_at TEXT,
            title TEXT NOT NULL,
            body TEXT NOT NULL,
            color INTEGER NOT NULL,
            is_pinned INTEGER
        )",
        NO_PARAMS,
    )?;

    let mut stmt = conn.prepare("SELECT * FROM notes WHERE archived_at IS NOT NULL")?;
    let notes_iter = stmt.query_map(NO_PARAMS, |row| {
        Ok(db::Note {
            id: row.get(0)?,
            created_at: row.get(1)?,
            updated_at: row.get(2)?,
            archived_at: row.get(3)?,
            title: row.get(4)?,
            body: row.get(5)?,
            color: row.get(6)?,
            is_pinned: row.get(7)?,
        })
    })?;
    let notes_found: Result<Vec<db::Note>, _> = notes_iter.collect();
    let notes_found = notes_found?;

    let ret: Message = notes::GuiNotes { notes: notes_found }.into();

    return Ok(ret);
}

pub fn delete_note(input: notes::GuiDeleteNote) -> Result<Message, BloomError> {
    let contents = std::fs::read_to_string("/proc/self/cmdline")?;
    let contents = contents.trim_end_matches('\x00');
    let db_dir = format!("/data/data/{}/databases", &contents);
    std::fs::create_dir_all(&db_dir)?;
    let db_path = format!("{}/bloom42.db", &db_dir);
    let conn = Connection::open(&db_path)?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS notes (
        id TEXT PRIMARY KEY NOT NULL,
        created_at TEXT NOT NULL,
        updated_at TEXT NOT NULL,
        archived_at TEXT,
        title TEXT NOT NULL,
        body TEXT NOT NULL,
        color INTEGER NOT NULL,
        is_pinned INTEGER
    )",
        NO_PARAMS,
    )?;

    conn.execute("DELETE FROM notes WHERE id = ?", params![input.id])?;

    let ret: Message = bloom_messages::kernel::Empty {}.into();

    return Ok(ret);
}

pub fn create_note(input: notes::GuiCreateNote) -> Result<Message, BloomError> {
    let contents = std::fs::read_to_string("/proc/self/cmdline")?;
    let contents = contents.trim_end_matches('\x00');
    let db_dir = format!("/data/data/{}/databases", &contents);
    std::fs::create_dir_all(&db_dir)?;
    let db_path = format!("{}/bloom42.db", &db_dir);
    let conn = Connection::open(&db_path)?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS notes (
        id TEXT PRIMARY KEY NOT NULL,
        created_at TEXT NOT NULL,
        updated_at TEXT NOT NULL,
        archived_at TEXT,
        title TEXT NOT NULL,
        body TEXT NOT NULL,
        color INTEGER NOT NULL,
        is_pinned INTEGER
    )",
        NO_PARAMS,
    )?;

    let now = chrono::Utc::now();
    let note = db::Note {
        id: uuid::Uuid::new_v4().to_hyphenated().to_string(),
        created_at: now,
        updated_at: now,
        archived_at: None,
        title: input.title,
        body: input.body,
        color: input.color,
        is_pinned: false,
    };

    conn.execute(
        "INSERT INTO notes (id, created_at, updated_at, archived_at, title, body, color, is_pinned)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?)",
        params![
            &note.id,
            &note.created_at,
            &note.updated_at,
            &note.archived_at,
            &note.title,
            &note.body,
            &note.color,
            &note.is_pinned,
        ],
    )?;

    let ret: Message = notes::GuiNote { note }.into();

    return Ok(ret);
}

pub fn update_note(input: notes::GuiUpdateNote) -> Result<Message, BloomError> {
    let contents = std::fs::read_to_string("/proc/self/cmdline")?;
    let contents = contents.trim_end_matches('\x00');
    let db_dir = format!("/data/data/{}/databases", &contents);
    std::fs::create_dir_all(&db_dir)?;
    let db_path = format!("{}/bloom42.db", &db_dir);
    let conn = Connection::open(&db_path)?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS notes (
        id TEXT PRIMARY KEY NOT NULL,
        created_at TEXT NOT NULL,
        updated_at TEXT NOT NULL,
        archived_at TEXT,
        title TEXT NOT NULL,
        body TEXT NOT NULL,
        color INTEGER NOT NULL,
        is_pinned INTEGER
    )",
        NO_PARAMS,
    )?;

    let mut note = input.note;
    note.updated_at = chrono::Utc::now();

    conn.execute(
        "UPDATE notes SET
            updated_at = ?,
            archived_at = ?,
            title = ?,
            body = ?,
            color = ?,
            is_pinned = ?
        WHERE id = ?",
        params![
            &note.updated_at,
            &note.archived_at,
            &note.title,
            &note.body,
            &note.color,
            &note.is_pinned,
            &note.id,
        ],
    )?;

    let ret: Message = notes::GuiNote { note }.into();

    return Ok(ret);
}
