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

    pub account_id: uuid::Uuid,

    pub token_hash: String,
    pub ip: String,
    pub user_agent: String,
}

impl Session {
    pub fn new() -> Self {
        let uuid = uuid::Uuid::new_v4();
        let now = chrono::Utc::now();
        return Session {
            id: uuid,
            created_at: now,
            updated_at: now,

            token_hash: String::new(),
            ip: String::new(),
            user_agent: String::new(),

            account_id: uuid,
        };
    }
}

impl Default for Session {
    fn default() -> Self {
        Self::new()
    }
}
