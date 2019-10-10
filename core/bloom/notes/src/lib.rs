use bloom_error::BloomError;
use bloom_messages::{notes, Message};
use bloom_models::notes::db;
use rusqlite::{Connection, NO_PARAMS};

pub fn list_notes(_: notes::GuiListNotes) -> Result<Message, BloomError> {
    let conn = Connection::open_in_memory()?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS notes (
            id TEXT PRIMARY KEY NOT NULL,
            created_at INTEGER NOT NULL,
            updated_at INTEGER NOT NULL,
            archived_at INTEGER,
            title TEXT NOT NULL,
            body TEXT NOT NULL,
            color INTEGER NOT NULL,
            is_pinned INTEGER
        )",
        NO_PARAMS,
    )?;

    let mut stmt = conn.prepare("SELECT * FROM notes WHERE archived_at IS NULL")?;
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
    let conn = Connection::open_in_memory()?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS notes (
            id TEXT PRIMARY KEY NOT NULL,
            created_at INTEGER NOT NULL,
            updated_at INTEGER NOT NULL,
            archived_at INTEGER,
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
    let conn = Connection::open_in_memory()?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS notes (
        id TEXT PRIMARY KEY NOT NULL,
        created_at INTEGER NOT NULL,
        updated_at INTEGER NOT NULL,
        archived_at INTEGER,
        title TEXT NOT NULL,
        body TEXT NOT NULL,
        color INTEGER NOT NULL,
        is_pinned INTEGER
    )",
        NO_PARAMS,
    )?;

    conn.execute("DELETE FROM notes WHERE id = $1", &[input.id])?;

    let ret: Message = bloom_messages::kernel::Empty {}.into();

    return Ok(ret);
}
