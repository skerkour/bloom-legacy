mod events;
mod commands;


use serde::{Serialize, Deserialize};
use diesel::{Queryable};
use diesel_as_jsonb::AsJsonb;
use actix::{Message, Handler};
use failure::Error;
use crate::{
    api,
    db::DbActor,
    db::schema::account_sessions,
    db::schema::account_sessions_events,
    services::common::events::EventMetadata,
};
use crate::error::KernelError;


pub use events::StartedV1;
pub use commands::Start;

pub const TOKEN_BCRYPT_COST: u32 = 5;
pub const TOKEN_MIN_LENGTH: u32 = 256;
pub const TOKEN_MAX_LENGTH: u32 = 512;


#[derive(Clone, Debug, Deserialize, Insertable, Queryable, Serialize)]
#[table_name = "account_sessions"]
pub struct Session {
    pub id: uuid::Uuid,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub deleted_at: Option<chrono::DateTime<chrono::Utc>>,
    pub version: i64,

    pub device: Device,
    pub ip: String,
    pub location: Location,
    pub token: String,

    pub account_id: uuid::Uuid,
}

impl Session {
    pub fn new() -> Self {
        let uuid = uuid::Uuid::new_v4();
        let now = chrono::Utc::now();
        return Session{
            id: uuid,
            created_at: now,
            updated_at: now,
            deleted_at: None,
            version: 0,

            device: Device{},
            ip: "".to_string(),
            location: Location{},
            token: "".to_string(),

            account_id: uuid,
        };
    }
}

#[derive(AsJsonb, Clone, Debug, Deserialize, Serialize)]
pub struct Device {}

#[derive(AsJsonb, Clone, Debug, Deserialize, Serialize)]
pub struct Location {}


#[derive(Clone, Debug, Deserialize, Insertable, Queryable, Serialize)]
#[table_name = "account_sessions_events"]
pub struct Event {
    pub id: uuid::Uuid,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub data: EventData,
    pub aggregate_id: uuid::Uuid,
    pub metadata: EventMetadata, // TODO: change
}

#[derive(Clone, Debug)]
pub enum Command {
    Start(Start),
}

#[derive(AsJsonb, Clone, Debug, Deserialize, Serialize)]
pub enum EventData {
    StartedV1(StartedV1),
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum NonPersistedData {
}
