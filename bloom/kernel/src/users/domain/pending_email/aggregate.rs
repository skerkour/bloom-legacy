use serde::{Serialize, Deserialize};
use diesel::{Queryable};
use crate::{
    db::schema::kernel_pending_emails,
};


#[derive(AsChangeset, Clone, Debug, Deserialize, Identifiable, Insertable, Queryable, Serialize)]
#[table_name = "kernel_pending_emails"]
#[changeset_options(treat_none_as_null = "true")]
pub struct PendingEmail {
    pub id: uuid::Uuid,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub deleted_at: Option<chrono::DateTime<chrono::Utc>>,
    pub version: i64,

    pub email: String,
    pub token: String, // hashed token
    pub trials: i64,

    pub user_id: uuid::Uuid,
}


impl PendingEmail {
    // create a new, unitialized PendingAccount
    pub fn new() -> Self {
        let now = chrono::Utc::now();
        return PendingEmail{
            id: uuid::Uuid::new_v4(),
            created_at: now,
            updated_at: now,
            deleted_at: None,
            version: 0,

            email: String::new(),
            token: String::new(),
            trials: 0,

            user_id: uuid::Uuid::new_v4(),
        };
    }
}

impl eventsourcing::Aggregate for PendingEmail {
    fn increment_version(&mut self) {
        self.version += 1;
    }

    fn update_updated_at(&mut self, timestamp: chrono::DateTime<chrono::Utc>) {
        self.updated_at = timestamp;
    }
}
