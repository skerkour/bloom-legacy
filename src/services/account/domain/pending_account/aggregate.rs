use serde::{Serialize, Deserialize};
use diesel::{Queryable};
use crate::{
    db::schema::account_pending_accounts,
};


#[derive(Clone, Debug, Deserialize, Insertable, Queryable, Serialize)]
#[table_name = "account_pending_accounts"]
pub struct PendingAccount {
    pub id: uuid::Uuid,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub deleted_at: Option<chrono::DateTime<chrono::Utc>>,
    pub version: i64,

    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub password: String, // hashed password
    pub token: String, // hashed verification code
    pub trials: i64,
}

impl PendingAccount {
    // create a new, unitialized PendingAccount
    pub fn new() -> Self {
        let now = chrono::Utc::now();
        return PendingAccount{
            id: uuid::Uuid::new_v4(),
            created_at: now,
            updated_at: now,
            deleted_at: None,
            version: 0,

            email: String::new(),
            first_name: String::new(),
            last_name: String::new(),
            password: String::new(),
            token: String::new(),
            trials: 0,
        };
    }
}

impl eventsourcing::Aggregate for PendingAccount {
    fn increment_version(&mut self) {
        self.version += 1;
    }

    fn update_updated_at(&mut self, timestamp: chrono::DateTime<chrono::Utc>) {
        self.updated_at = timestamp;
    }
}


// #[derive(Clone, Debug)]
// pub enum Command {
//     Create(Create),
//     Verify(Verify),
//     ResendCode(ResendCode),
//     CompleteRegistration(CompleteRegistration),
// }
