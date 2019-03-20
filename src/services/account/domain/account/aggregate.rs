use serde::{Serialize, Deserialize};
use diesel::{Queryable};
use crate::{
    db::schema::account_accounts,
};

#[derive(Clone, Debug, Deserialize, Insertable, Queryable, Serialize)]
#[table_name = "account_accounts"]
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
    pub recovery_id: Option<uuid::Uuid>,
    pub recovery_token: Option<String>,
    pub username: String,
}


impl Account {
    pub fn new() -> Self {
        let now = chrono::Utc::now();
        return Account{
            id: uuid::Uuid::new_v4(),
            created_at: now,
            updated_at: now,
            deleted_at: None,
            version: 0,

            avatar_url: "".to_string(),
            email: "".to_string(),
            first_name: "".to_string(),
            is_admin: false,
            last_name: "".to_string(),
            password: "".to_string(),
            recovery_id: None,
            recovery_token: None,
            username: "".to_string(),
        };
    }
}

impl eventsourcing::Aggregate for Account {
    fn increment_version(&mut self) {
        self.version += 1;
    }
}
