use crate::db::schema::accounts;
use diesel::Queryable;
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
#[table_name = "accounts"]
#[changeset_options(treat_none_as_null = "true")]
pub struct Account {
    pub id: uuid::Uuid,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,

    pub avatar_id: Option<String>,
    pub username: String,
    pub display_name: String,
    pub bio: String,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub is_admin: bool,
    pub disabled_at: Option<chrono::DateTime<chrono::Utc>>,
    pub auth_key_hash: String,
}

impl Account {
    // create a new, unitialized Account
    pub fn new() -> Self {
        let now = chrono::Utc::now();
        return Account {
            id: uuid::Uuid::new_v4(),
            created_at: now,
            updated_at: now,

            avatar_id: None,
            username: String::new(),
            display_name: String::new(),
            bio: String::new(),
            email: String::new(),
            first_name: String::new(),
            last_name: String::new(),
            is_admin: false,
            disabled_at: None,
            auth_key_hash: String::new(),
        };
    }
}

impl Default for Account {
    fn default() -> Self {
        Self::new()
    }
}
