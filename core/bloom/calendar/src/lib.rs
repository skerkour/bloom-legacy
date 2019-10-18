use bloom_error::BloomError;
use bloom_messages::{calendar, Message};
use bloom_models::calendar::db;
use rusqlite::{params, Connection, NO_PARAMS};
use chrono::Datelike;

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
    let start_at = input
        .start_at
        .unwrap_or_else(|| now.with_day(1).unwrap());
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

    let mut stmt = conn.prepare("SELECT * FROM calendar_events
        WHERE
            (strftime('%s', start_at) BETWEEN strftime('%s', $1) AND strftime('%s', $2))
            OR (strftime('%s', end_at) BETWEEN strftime('%s', $1) AND strftime('%s', $2))")?;
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
