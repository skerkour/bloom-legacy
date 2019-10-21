use bloom_error::BloomError;
use bloom_messages::{contacts, Message};
use bloom_models::contacts::db;
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

pub fn list_contacts(_: contacts::GuiListContacts) -> Result<Message, BloomError> {
    let conn = Connection::open(db_path())?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS contacts (
            id TEXT PRIMARY KEY NOT NULL,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL,
            first_name TEXT NOT NULL,
            last_name TEXT NOT NULL,
            notes TEXT NOT NULL,
            addresses TEXT NOT NULL,
            birthday TEXT,
            organizations TEXT NOT NULL,
            emails TEXT NOT NULL,
            phones TEXT NOT NULL,
            websites TEXT NOT NULL
        )",
        NO_PARAMS,
    )?;

    let mut stmt = conn.prepare("SELECT * FROM contacts")?;
    let contacts_iter = stmt.query_map(params![], |row| {
        Ok(db::Conatct {
            id: row.get(0)?,
            created_at: row.get(1)?,
            updated_at: row.get(2)?,
            first_name: row.get(3)?,
            last_name: row.get(4)?,
            notes: row.get(5)?,
            addresses: row.get(6)?,
            birthday: row.get(7)?,
            organizations: row.get(8)?,
            emails: row.get(9)?,
            phones: row.get(10)?,
            websites: row.get(11)?,
        })
    })?;
    let contacts: Result<Vec<db::Conatct>, _> = contacts_iter.collect();
    let contacts = contacts?;

    let ret: Message = contacts::GuiContacts { contacts }.into();

    return Ok(ret);
}

pub fn create_contact(input: contacts::GuiCreateContact) -> Result<Message, BloomError> {
    let conn = Connection::open(db_path())?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS contacts (
            id TEXT PRIMARY KEY NOT NULL,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL,
            first_name TEXT NOT NULL,
            last_name TEXT NOT NULL,
            notes TEXT NOT NULL,
            addresses TEXT NOT NULL,
            birthday TEXT,
            organizations TEXT NOT NULL,
            emails TEXT NOT NULL,
            phones TEXT NOT NULL,
            websites TEXT NOT NULL
        )",
        NO_PARAMS,
    )?;

    let now = chrono::Utc::now();
    let contact = db::Conatct {
        id: uuid::Uuid::new_v4().to_hyphenated().to_string(),
        created_at: now,
        updated_at: now,
        first_name: input.first_name,
        last_name: input.last_name,
        notes: input.notes,
        addresses: input.addresses,
        birthday: input.birthday,
        organizations: input.organizations,
        emails: input.emails,
        phones: input.phones,
        websites: input.websites,
    };

    conn.execute(
        "INSERT INTO contacts
            (id,
            created_at,
            updated_at,
            first_name,
            last_name,
            notes,
            addresses,
            birthday,
            organizations,
            emails,
            phones,
            websites)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)",
        params![
            &contact.id,
            &contact.created_at,
            &contact.updated_at,
            &contact.first_name,
            &contact.last_name,
            &contact.notes,
            &contact.addresses,
            &contact.birthday,
            &contact.organizations,
            &contact.emails,
            &contact.phones,
            &contact.websites,
        ],
    )?;

    let ret: Message = contacts::GuiContact { contact }.into();

    return Ok(ret);
}

pub fn delete_contact(input: contacts::GuiDeleteContact) -> Result<Message, BloomError> {
    let conn = Connection::open(db_path())?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS contacts (
            id TEXT PRIMARY KEY NOT NULL,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL,
            first_name TEXT NOT NULL,
            last_name TEXT NOT NULL,
            notes TEXT NOT NULL,
            addresses TEXT NOT NULL,
            birthday TEXT,
            organizations TEXT NOT NULL,
            emails TEXT NOT NULL,
            phones TEXT NOT NULL,
            websites TEXT NOT NULL
        )",
        NO_PARAMS,
    )?;

    conn.execute("DELETE FROM contacts WHERE id = ?", params![input.id])?;

    let ret: Message = bloom_messages::kernel::Empty {}.into();

    return Ok(ret);
}

pub fn update_contact(input: contacts::GuiUpdateContact) -> Result<Message, BloomError> {
    let conn = Connection::open(db_path())?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS contacts (
            id TEXT PRIMARY KEY NOT NULL,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL,
            first_name TEXT NOT NULL,
            last_name TEXT NOT NULL,
            notes TEXT NOT NULL,
            addresses TEXT NOT NULL,
            birthday TEXT,
            organizations TEXT NOT NULL,
            emails TEXT NOT NULL,
            phones TEXT NOT NULL,
            websites TEXT NOT NULL
        )",
        NO_PARAMS,
    )?;

    let mut contact = input.contact;
    contact.updated_at = chrono::Utc::now();

    conn.execute(
        "UPDATE calendar_events SET
            updated_at = $1,
            first_name = $2,
            last_name = $3,
            notes = $4,
            addresses = $5,
            birthday = $6,
            organizations = $7,
            emails = $8,
            phones = $9,
            websites = $10,
        WHERE id = $11",
        params![
            &contact.updated_at,
            &contact.first_name,
            &contact.last_name,
            &contact.notes,
            &contact.addresses,
            &contact.birthday,
            &contact.organizations,
            &contact.emails,
            &contact.phones,
            &contact.websites,
            &contact.id,
        ],
    )?;

    let ret: Message = contacts::GuiContact { contact }.into();

    return Ok(ret);
}
