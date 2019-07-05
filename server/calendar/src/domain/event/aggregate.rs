use diesel::Queryable;
use kernel::db::schema::calendar_events;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(
    AsChangeset,
    Clone,
    Debug,
    Deserialize,
    Identifiable,
    Insertable,
    Queryable,
    QueryableByName,
    Serialize,
)]
#[table_name = "calendar_events"]
#[changeset_options(treat_none_as_null = "true")]
pub struct CalendarEvent {
    pub id: Uuid,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub deleted_at: Option<chrono::DateTime<chrono::Utc>>,
    pub version: i64,

    pub title: String,
    pub description: String,
    pub start_at: chrono::DateTime<chrono::Utc>,
    pub end_at: chrono::DateTime<chrono::Utc>,

    pub owner_id: Uuid,
}

impl CalendarEvent {
    // create a new, unitialized CalendarEvent
    pub fn new() -> Self {
        let now = chrono::Utc::now();
        return CalendarEvent {
            id: Uuid::new_v4(),
            created_at: now,
            updated_at: now,
            deleted_at: None,
            version: 0,
            owner_id: Uuid::new_v4(),

            title: String::new(),
            description: String::new(),
            start_at: now,
            end_at: now,
        };
    }
}

impl eventsourcing::Aggregate for CalendarEvent {
    fn increment_version(&mut self) {
        self.version += 1;
    }

    fn update_updated_at(&mut self, timestamp: chrono::DateTime<chrono::Utc>) {
        self.updated_at = timestamp;
    }
}

impl Default for CalendarEvent {
    fn default() -> Self {
        Self::new()
    }
}
