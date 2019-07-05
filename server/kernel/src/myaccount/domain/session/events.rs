use crate::{events::EventMetadata};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Event {
    pub id: uuid::Uuid,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub data: EventData,
    pub aggregate_id: uuid::Uuid,
    pub metadata: EventMetadata,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum EventData {
    StartedV1(StartedV1),
    SignedOutV1,
    RevokedV1(RevokedV1),
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct StartedV1 {
    pub id: uuid::Uuid,
    pub account_id: uuid::Uuid,
    pub token_hash: String,
    pub token_plaintext: String,
    pub ip: String,
    pub location: super::Location,
    pub device: super::Device,
}

#[derive(Copy, Clone, Debug, Deserialize, Serialize)]
pub struct RevokedV1{
    pub reason: RevokedReason,
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

    fn apply(&self, aggregate: &mut Self::Aggregate) {
        match self.data {
            // StartedV1
            EventData::StartedV1(ref data) => {
                aggregate.id =  data.id;
                aggregate.created_at =  self.timestamp;
                aggregate.updated_at =  self.timestamp;
                aggregate.deleted_at =  None;
                aggregate.version =  0;
                aggregate.device =  data.device.clone();
                aggregate.ip =  data.ip.clone();
                aggregate.location =  Some(data.location.clone());
                aggregate.token =  data.token.clone();
                aggregate.account_id =  data.account_id;
            },
            // SignedOutV1 | RevokedV1
            EventData::SignedOutV1 | EventData::RevokedV1(_) => {
                aggregate.deleted_at = Some(self.timestamp);
            },
        }
    }

    fn timestamp(&self) -> chrono::DateTime<chrono::Utc> {
        return self.timestamp;
    }
}
