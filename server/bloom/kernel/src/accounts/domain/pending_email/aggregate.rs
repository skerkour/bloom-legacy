use crate::db::schema::kernel_pending_emails;
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
#[table_name = "kernel_pending_emails"]
#[changeset_options(treat_none_as_null = "true")]
pub struct PendingEmail {
    pub id: uuid::Uuid,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub version: i64,

    pub email: String,
    pub token: String, // hashed token
    pub trials: i64,

    pub account_id: uuid::Uuid,
}

impl PendingEmail {
    // create a new, unitialized PendingAccount
    pub fn new() -> Self {
        let now = chrono::Utc::now();
        return PendingEmail {
            id: uuid::Uuid::new_v4(),
            created_at: now,
            updated_at: now,
            version: 0,

            email: String::new(),
            token: String::new(),
            trials: 0,

            account_id: uuid::Uuid::new_v4(),
        };
    }
}

impl Default for PendingEmail {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub enum VerificationFailedReason {
    CodeNotValid,
    CodeExpired,
    TooManyTrials,
}

impl ToString for VerificationFailedReason {
    fn to_string(&self) -> String {
        match self {
            VerificationFailedReason::CodeNotValid => "Code is not valid.".to_string(),
            VerificationFailedReason::CodeExpired => {
                "Code has expired. Please try again".to_string()
            }
            VerificationFailedReason::TooManyTrials => {
                "Maximum number of trials reached. Please ry again".to_string()
            }
        }
    }
}
