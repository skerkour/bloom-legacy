use crate::db::schema::kernel_accounts;
use diesel::Queryable;
use serde::{Deserialize, Serialize};

#[derive(AsChangeset, Clone, Debug, Deserialize, Identifiable, Insertable, Queryable, Serialize)]
#[table_name = "kernel_accounts"]
#[changeset_options(treat_none_as_null = "true")]
pub struct Account {
    pub id: uuid::Uuid,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub deleted_at: Option<chrono::DateTime<chrono::Utc>>,
    pub version: i64,

    pub avatar_url: String,
    pub email: String,
    pub first_name: String,
    pub is_admin: bool,
    pub last_name: String,
    pub password: String, // hashed password
    pub password_reset_id: Option<uuid::Uuid>,
    pub password_reset_token: Option<String>,
    pub username: String,
    pub is_disabled: bool,
    pub bio: String,
    pub display_name: String,
}

impl Account {
    // create a new, unitialized Account
    pub fn new() -> Self {
        let now = chrono::Utc::now();
        return Account {
            id: uuid::Uuid::new_v4(),
            created_at: now,
            updated_at: now,
            deleted_at: None,
            version: 0,

            avatar_url: String::new(),
            email: String::new(),
            first_name: String::new(),
            is_admin: false,
            last_name: String::new(),
            password: String::new(),
            password_reset_id: None,
            password_reset_token: None,
            username: String::new(),
            is_disabled: false,
            bio: String::new(),
            display_name: String::new(),
        };
    }
}

impl eventsourcing::Aggregate for Account {
    fn increment_version(&mut self) {
        self.version += 1;
    }

    fn update_updated_at(&mut self, timestamp: chrono::DateTime<chrono::Utc>) {
        self.updated_at = timestamp;
    }
}
