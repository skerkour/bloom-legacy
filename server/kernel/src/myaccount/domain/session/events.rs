use serde::{Deserialize, Serialize};
use diesel::{Queryable};
use diesel_as_jsonb::AsJsonb;
use crate::{
    db::schema::kernel_sessions_events,
    events::EventMetadata,
};


#[derive(Clone, Debug, Deserialize, Insertable, Queryable, Serialize)]
#[table_name = "kernel_sessions_events"]
pub struct Event {
    pub id: uuid::Uuid,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub data: EventData,
    pub aggregate_id: uuid::Uuid,
    pub metadata: EventMetadata,
}

#[derive(AsJsonb, Clone, Debug, Deserialize, Serialize)]
pub enum EventData {
    StartedV1(StartedV1),
    SignedOutV1,
    RevokedV1(RevokedReason),
}



#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct StartedV1 {
    pub id: uuid::Uuid,
    pub account_id: uuid::Uuid,
    pub token: String,
    pub ip: String,
    pub location: super::Location,
    pub device: super::Device,
}


#[derive(Copy, Clone, Debug, Deserialize, Serialize)]
pub enum RevokedReason {
    Manually,
    PasswordUpdated,
    PasswordReseted,
    AccountDisabled,
}

impl eventsourcing::Event for Event {
    type Aggregate = super::Session;

    fn apply(&self, aggregate: Self::Aggregate) -> Self::Aggregate {
        match self.data {
            // StartedV1
            EventData::StartedV1(ref data) => super::Session{
                id: data.id,
                created_at: self.timestamp,
                updated_at: self.timestamp,
                deleted_at: None,
                version: 0,
                device: data.device.clone(),
                ip: data.ip.clone(),
                location: Some(data.location.clone()),
                token: data.token.clone(),
                account_id: data.account_id,
            },
            // SignedOutV1 | RevokedV1
            EventData::SignedOutV1 | EventData::RevokedV1(_) => super::Session{
                deleted_at: Some(self.timestamp),
                ..aggregate
            },
        }
    }

    fn timestamp(&self) -> chrono::DateTime<chrono::Utc> {
        return self.timestamp;
    }
}
