use diesel::Queryable;
use eventsourcing::Aggregate;
use kernel::db::schema::bitflow_profiles;
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
#[table_name = "bitflow_profiles"]
#[changeset_options(treat_none_as_null = "true")]
pub struct Profile {
    pub id: uuid::Uuid,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub version: i64,

    pub account_id: uuid::Uuid,
    pub download_folder_id: uuid::Uuid,
}

impl Profile {
    // create a new, unitialized Profile
    pub fn new() -> Self {
        let now = chrono::Utc::now();
        return Profile {
            id: uuid::Uuid::new_v4(),
            created_at: now,
            updated_at: now,
            version: 0,

            account_id: uuid::Uuid::new_v4(),
            download_folder_id: uuid::Uuid::new_v4(),
        };
    }
}

impl Default for Profile {
    fn default() -> Self {
        Self::new()
    }
}
