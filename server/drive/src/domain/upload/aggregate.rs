use diesel::Queryable;
use kernel::db::schema::drive_uploads;
use serde::{Deserialize, Serialize};

#[derive(AsChangeset, Clone, Debug, Deserialize, Identifiable, Insertable, Queryable, Serialize)]
#[table_name = "drive_uploads"]
#[changeset_options(treat_none_as_null = "true")]
pub struct Upload {
    pub id: uuid::Uuid,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub deleted_at: Option<chrono::DateTime<chrono::Utc>>,
    pub version: i64,

    pub file_id: uuid::Uuid,
    pub file_name: String,
    pub parent_id: Option<uuid::Uuid>,
    pub size: i64,
    #[serde(rename = "type")]
    pub type_: String, // MIME type

    pub owner_id: uuid::Uuid,
}

impl Upload {
    // create a new, unitialized Upload
    pub fn new() -> Self {
        let now = chrono::Utc::now();
        return Upload {
            id: uuid::Uuid::new_v4(),
            created_at: now,
            updated_at: now,
            deleted_at: None,
            version: 0,

            file_id: uuid::Uuid::new_v4(),
            file_name: String::new(),
            parent_id: None,
            size: 0,
            type_: String::new(),

            owner_id: uuid::Uuid::new_v4(),
        };
    }
}

impl eventsourcing::Aggregate for Upload {
    fn increment_version(&mut self) {
        self.version += 1;
    }

    fn update_updated_at(&mut self, timestamp: chrono::DateTime<chrono::Utc>) {
        self.updated_at = timestamp;
    }
}
