use diesel::Queryable;
use eventsourcing::Aggregate;
use kernel::db::schema::notes_notes;
use serde::{Deserialize, Serialize};

#[derive(
    Aggregate,
    AsChangeset,
    Clone,
    Debug,
    Deserialize,
    Identifiable,
    Insertable,
    Queryable,
    Serialize,
)]
#[table_name = "notes_notes"]
#[changeset_options(treat_none_as_null = "true")]
pub struct Note {
    pub id: uuid::Uuid,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub version: i64,

    pub archived_at: Option<chrono::DateTime<chrono::Utc>>,
    pub body: String,
    pub removed_at: Option<chrono::DateTime<chrono::Utc>>,
    pub title: String,

    pub owner_id: uuid::Uuid,
}

impl Note {
    // create a new, unitialized note
    pub fn new() -> Self {
        let now = chrono::Utc::now();
        return Note {
            id: uuid::Uuid::new_v4(),
            created_at: now,
            updated_at: now,
            version: 0,

            archived_at: None,
            body: String::new(),
            removed_at: None,
            title: String::new(),

            owner_id: uuid::Uuid::new_v4(),
        };
    }
}

impl Default for Note {
    fn default() -> Self {
        Self::new()
    }
}
