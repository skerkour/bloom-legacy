use serde::{Deserialize, Serialize};
use diesel::{Queryable};
use diesel_as_jsonb::AsJsonb;
use crate::{
    db::schema::account_pending_emails_events,
    services::common::events::EventMetadata,
};



#[derive(Clone, Debug, Deserialize, Insertable, Queryable, Serialize)]
#[table_name = "account_pending_emails_events"]
pub struct Event {
    pub id: uuid::Uuid,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub data: EventData,
    pub aggregate_id: uuid::Uuid,
    pub metadata: EventMetadata, // TODO: change
}

#[derive(AsJsonb, Clone, Debug, Deserialize, Serialize)]
pub enum EventData {
    CreatedV1(CreatedV1),
    DeletedV1,
    VerificationFailedV1(VerificationFailedV1),
    VerificationSucceededV1,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreatedV1 {
    pub id: uuid::Uuid,
    pub email: String,
    pub token: String,
    pub account_id: uuid::Uuid,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct VerificationFailedV1 {
    pub reason: String,
}

impl eventsourcing::Event for Event {
    type Aggregate = super::PendingEmail;

    fn apply(&self, aggregate: Self::Aggregate) -> Self::Aggregate {
        match self.data {
            // CreatedV1
            EventData::CreatedV1(ref data) => super::PendingEmail{
                id: data.id,
                created_at: self.timestamp,
                updated_at: self.timestamp,
                deleted_at: None,
                version: 0,
                email: data.email.clone(),
                token: data.token.clone(),
                trials: 0,
                account_id: data.account_id,
            },
            // VerificationSucceededV1
            EventData::VerificationSucceededV1 => aggregate,
            // VerificationFailedV1
            EventData::VerificationFailedV1(_) => super::PendingEmail{
                trials: aggregate.trials + 1,
                ..aggregate
            },
            // PendingEmail
            EventData::DeletedV1 => super::PendingEmail{
                deleted_at: Some(self.timestamp),
                ..aggregate
            },
        }
    }

    fn timestamp(&self) -> chrono::DateTime<chrono::Utc> {
        return self.timestamp;
    }
}
