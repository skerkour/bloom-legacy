use serde::{Deserialize, Serialize};
use diesel::{Queryable};
use diesel_as_jsonb::AsJsonb;
use crate::{
    db::schema::account_pending_accounts_events,
    services::common::events::EventMetadata,
};



#[derive(Clone, Debug, Deserialize, Insertable, Queryable, Serialize)]
#[table_name = "account_pending_accounts_events"]
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
    VerificationFailedV1(VerificationFailedV1),
    VerificationSucceededV1,
    RegistrationCompletedV1,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreatedV1 {
    pub id: uuid::Uuid,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
    pub token: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct VerificationFailedV1 {
    pub reason: String,
}

impl eventsourcing::Event for Event {
    type Aggregate = super::PendingAccount;

    fn apply(&self, aggregate: Self::Aggregate) -> Self::Aggregate {
        match self.data {
            // CreatedV1
            EventData::CreatedV1(ref data) => super::PendingAccount{
                id: data.id,
                created_at: self.timestamp,
                updated_at: self.timestamp,
                deleted_at: None,
                version: 0,
                email: data.email.clone(),
                first_name: data.first_name.clone(),
                last_name: data.last_name.clone(),
                password: data.password.clone(),
                token: data.token.clone(),
                trials: 0,
            },
            EventData::VerificationSucceededV1 => aggregate,
            EventData::VerificationFailedV1(_) => super::PendingAccount{
                trials: aggregate.trials + 1,
                ..aggregate
            },
            EventData::RegistrationCompletedV1 => super::PendingAccount{
                deleted_at: Some(self.timestamp),
                ..aggregate
            },
        }
    }

    fn timestamp(&self) -> chrono::DateTime<chrono::Utc> {
        return self.timestamp;
    }
}
