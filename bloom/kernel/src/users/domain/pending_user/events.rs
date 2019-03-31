use serde::{Deserialize, Serialize};
use diesel::{Queryable};
use diesel_as_jsonb::AsJsonb;
use crate::{
    db::schema::kernel_pending_users_events,
    events::EventMetadata,
};



#[derive(Clone, Debug, Deserialize, Insertable, Queryable, Serialize)]
#[table_name = "kernel_pending_users_events"]
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
    VerificationFailedV1(VerificationFailedReason),
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
            VerificationFailedReason::CodeExpired => "Code has expired. Please create another user.".to_string(),
            VerificationFailedReason::TooManyTrials => "Maximum number of trials reached. Please create another user.".to_string(),
        }
    }
}

impl eventsourcing::Event for Event {
    type Aggregate = super::PendingUser;

    fn apply(&self, aggregate: Self::Aggregate) -> Self::Aggregate {
        match self.data {
            // CreatedV1
            EventData::CreatedV1(ref data) => super::PendingUser{
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
                verified: false,
            },
            EventData::VerificationSucceededV1 => super::PendingUser{
                verified: true,
                ..aggregate
            },
            EventData::VerificationFailedV1(_) => super::PendingUser{
                trials: aggregate.trials + 1,
                ..aggregate
            },
            EventData::RegistrationCompletedV1 => super::PendingUser{
                deleted_at: Some(self.timestamp),
                ..aggregate
            },
        }
    }

    fn timestamp(&self) -> chrono::DateTime<chrono::Utc> {
        return self.timestamp;
    }
}
