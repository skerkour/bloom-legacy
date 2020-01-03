use bloom_error::BloomError;
use bloom_messages::{calendar, Message};
use bloom_models::calendar::db;
use chrono::Datelike;
use rusqlite::{params, Connection, NO_PARAMS};

#[cfg(target_os = "android")]
fn home_dir() -> String {
    let contents = std::fs::read_to_string("/proc/self/cmdline").unwrap();
    let contents = contents.trim_end_matches('\x00');
    return format!("/data/data/{}", &contents);
}

#[cfg(not(target_os = "android"))]
fn home_dir() -> String {
    let home = dirs::home_dir();
    match home {
        Some(res) => format!("{}/.bloom", res.to_str().unwrap()),
        None => "".to_string(),
    }
}

fn db_dir() -> String {
    let path = format!("{}/db", home_dir());
    let _ = std::fs::create_dir_all(&path);
    return path;
}

fn db_path() -> String {
    return format!("{}/bloom42.db", db_dir());
}

pub fn list_events(input: calendar::GuiListEvents) -> Result<Message, BloomError> {
    let now = chrono::Utc::now();
    let start_at = input.start_at.unwrap_or_else(|| now.with_day(1).unwrap());
    let end_at = input
        .end_at
        .unwrap_or_else(|| start_at + chrono::Duration::days(31));

    bloom_validators::calendar::event_dates(start_at, end_at)?;

    let conn = Connection::open(db_path())?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS calendar_events (
            id TEXT PRIMARY KEY NOT NULL,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL,
            title TEXT NOT NULL,
            description TEXT NOT NULL,
            start_at TEXT NOT NULL,
            end_at TEXT NOT NULL
        )",
        NO_PARAMS,
    )?;

    let mut stmt = conn.prepare(
        "SELECT * FROM calendar_events
        WHERE
            (strftime('%s', start_at) BETWEEN strftime('%s', $1) AND strftime('%s', $2))
            OR (strftime('%s', end_at) BETWEEN strftime('%s', $1) AND strftime('%s', $2))",
    )?;
    let events_iter = stmt.query_map(params![start_at, end_at], |row| {
        Ok(db::Event {
            id: row.get(0)?,
            created_at: row.get(1)?,
            updated_at: row.get(2)?,
            title: row.get(3)?,
            description: row.get(4)?,
            start_at: row.get(5)?,
            end_at: row.get(6)?,
        })
    })?;
    let events: Result<Vec<db::Event>, _> = events_iter.collect();
    let events = events?;

    let ret: Message = calendar::GuiEvents { events }.into();

    return Ok(ret);
}

pub fn create_event(input: calendar::GuiCreateEvent) -> Result<Message, BloomError> {
    bloom_validators::calendar::event_dates(input.start_at, input.end_at)?;

    let conn = Connection::open(db_path())?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS calendar_events (
            id TEXT PRIMARY KEY NOT NULL,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL,
            title TEXT NOT NULL,
            description TEXT NOT NULL,
            start_at TEXT NOT NULL,
            end_at TEXT NOT NULL
        )",
        NO_PARAMS,
    )?;

    let now = chrono::Utc::now();
    let event = db::Event {
        id: uuid::Uuid::new_v4().to_hyphenated().to_string(),
        created_at: now,
        updated_at: now,
        title: input.title,
        description: input.description,
        start_at: input.start_at,
        end_at: input.end_at,
    };

    conn.execute(
        "INSERT INTO calendar_events (id, created_at, updated_at, title, description, start_at, end_at)
            VALUES (?, ?, ?, ?, ?, ?, ?)",
        params![
            &event.id,
            &event.created_at,
            &event.updated_at,
            &event.title,
            &event.description,
            &event.start_at,
            &event.end_at,
        ],
    )?;

    let ret: Message = calendar::GuiEvent { event }.into();

    return Ok(ret);
}

pub fn delete_event(input: calendar::GuiDeleteEvent) -> Result<Message, BloomError> {
    let conn = Connection::open(db_path())?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS calendar_events (
            id TEXT PRIMARY KEY NOT NULL,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL,
            title TEXT NOT NULL,
            description TEXT NOT NULL,
            start_at TEXT NOT NULL,
            end_at TEXT NOT NULL
        )",
        NO_PARAMS,
    )?;

    conn.execute(
        "DELETE FROM calendar_events WHERE id = ?",
        params![input.id],
    )?;

    let ret: Message = bloom_messages::kernel::Empty {}.into();

    return Ok(ret);
}

pub fn update_event(input: calendar::GuiUpdateEvent) -> Result<Message, BloomError> {
    let conn = Connection::open(db_path())?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS calendar_events (
            id TEXT PRIMARY KEY NOT NULL,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL,
            title TEXT NOT NULL,
            description TEXT NOT NULL,
            start_at TEXT NOT NULL,
            end_at TEXT NOT NULL
        )",
        NO_PARAMS,
    )?;

    let mut event = input.event;
    event.updated_at = chrono::Utc::now();

    conn.execute(
        "UPDATE calendar_events SET
            updated_at = ?,
            title = ?,
            description = ?,
            start_at = ?,
            end_at = ?
        WHERE id = ?",
        params![
            &event.updated_at,
            &event.title,
            &event.description,
            &event.start_at,
            &event.end_at,
            &event.id,
        ],
    )?;

    let ret: Message = calendar::GuiEvent { event }.into();

    return Ok(ret);
}
