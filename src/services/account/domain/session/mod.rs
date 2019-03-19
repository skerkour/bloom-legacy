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
};
use crate::error::KernelError;


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

#[derive(AsJsonb, Clone, Debug, Deserialize, Serialize)]
pub enum Device {}

#[derive(AsJsonb, Clone, Debug, Deserialize, Serialize)]
pub enum Location {}


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
}

#[derive(AsJsonb, Clone, Debug, Deserialize, Serialize)]
pub enum EventData {
}

#[derive(AsJsonb, Clone, Debug, Deserialize, Serialize)]
pub struct EventMetadata {
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum NonPersistedData {
}
