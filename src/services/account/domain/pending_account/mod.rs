mod commands;
mod events;

use serde::{Serialize, Deserialize};
use diesel::{Queryable};
use diesel_as_jsonb::AsJsonb;
use crate::{
    db::schema::account_pending_accounts,
    db::schema::account_pending_accounts_events,
};


pub use commands::{
    CompleteRegistration,
    Create,
    ResendCode,
    Verify,
};
pub use events::{
    CodeResentV1,
    CreatedV1,
    RegistrationCompletedV1,
    // VerificationFailedV1,
    // VerificationSucceededV1,
};

pub const TOKEN_BCRYPT_COST: u32 = 11;


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


#[derive(Clone, Debug, Deserialize, Insertable, Queryable, Serialize)]
#[table_name = "account_pending_accounts_events"]
pub struct Event {
    pub id: uuid::Uuid,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub data: EventData,
    pub aggregate_id: uuid::Uuid,
    pub metadata: EventMetadata, // TODO: change
}


#[derive(Clone, Debug)]
pub enum Command {
    Create(Create),
    Verify(Verify),
    ResendCode(ResendCode),
    CompleteRegistration(CompleteRegistration),
}

#[derive(AsJsonb, Clone, Debug, Deserialize, Serialize)]
pub enum EventData {
    CreatedV1(CreatedV1),
    CodeResentV1,
    VerificationFailedV1,
    VerificationSucceededV1,
    RegistrationCompletedV1,
}

#[derive(AsJsonb, Clone, Debug, Deserialize, Serialize)]
pub struct EventMetadata {
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum NonPersistedData {
}
