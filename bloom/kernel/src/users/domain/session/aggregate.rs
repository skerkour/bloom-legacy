use serde::{Serialize, Deserialize};
use diesel::{Queryable};
use crate::{
    db::schema::kernel_sessions,
};
use diesel_as_jsonb::AsJsonb;

#[derive(AsChangeset, Clone, Debug, Deserialize, Identifiable, Insertable, Queryable, Serialize)]
#[table_name = "kernel_sessions"]
#[changeset_options(treat_none_as_null = "true")]
pub struct Session {
    pub id: uuid::Uuid,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub deleted_at: Option<chrono::DateTime<chrono::Utc>>,
    pub version: i64,

    pub device: Device,
    pub ip: String,
    pub location: Option<Location>,
    pub token: String,

    pub user_id: uuid::Uuid,
}

impl Session {
    pub fn new() -> Self {
        let uuid = uuid::Uuid::new_v4();
        let now = chrono::Utc::now();
        return Session{
            id: uuid,
            created_at: now,
            updated_at: now,
            deleted_at: None,
            version: 0,

            device: Device{},
            ip: "".to_string(),
            location: None,
            token: "".to_string(),

            user_id: uuid,
        };
    }
}

impl eventsourcing::Aggregate for Session {
    fn increment_version(&mut self) {
        self.version += 1;
    }

    fn update_updated_at(&mut self, timestamp: chrono::DateTime<chrono::Utc>) {
        self.updated_at = timestamp;
    }
}


#[derive(AsJsonb, Clone, Debug, Deserialize, Serialize)]
pub struct Device {}

#[derive(AsJsonb, Clone, Debug, Deserialize, Serialize)]
pub struct Location {}
