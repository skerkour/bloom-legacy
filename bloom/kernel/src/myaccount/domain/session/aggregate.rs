use crate::db::schema::kernel_sessions;
use diesel::Queryable;
use diesel_as_jsonb::AsJsonb;
use eventsourcing::Aggregate;
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
#[table_name = "kernel_sessions"]
#[changeset_options(treat_none_as_null = "true")]
pub struct Session {
    pub id: uuid::Uuid,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,

    pub device: Device,
    pub ip: String,
    pub location: Option<Location>,
    pub token: String,

    pub account_id: uuid::Uuid,
}

impl Session {
    pub fn new() -> Self {
        let uuid = uuid::Uuid::new_v4();
        let now = chrono::Utc::now();
        return Session {
            id: uuid,
            created_at: now,
            updated_at: now,
            version: 0,

            device: Device {},
            ip: "".to_string(),
            location: None,
            token: "".to_string(),

            account_id: uuid,
        };
    }
}

impl Default for Session {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(AsJsonb, Clone, Debug, Deserialize, Serialize)]
pub struct Device {}

#[derive(AsJsonb, Clone, Debug, Deserialize, Serialize)]
pub struct Location {}

#[derive(Copy, Clone, Debug, Deserialize, Serialize)]
pub enum RevokedReason {
    Manually,
    PasswordUpdated,
    PasswordReseted,
    AccountDisabled,
}
