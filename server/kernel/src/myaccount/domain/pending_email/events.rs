use crate::events::EventMetadata;
use serde::{Deserialize, Serialize};
use std::string::ToString;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Event {
    pub id: uuid::Uuid,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub data: EventData,
    pub aggregate_id: uuid::Uuid,
    pub metadata: EventMetadata, // TODO: change
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum EventData {
    CreatedV1(CreatedV1),
    DeletedV1,
    VerificationFailedV1(VerificationFailedReason),
    VerificationSucceededV1,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreatedV1 {
    pub id: uuid::Uuid,
    pub email: String,
    pub token: String,
    pub account_id: uuid::Uuid,
    pub code: String,
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

impl eventsourcing::Event for Event {
    type Aggregate = super::PendingEmail;

    fn apply(&self, aggregate: &mut Self::Aggregate) {
        match self.data {
            // CreatedV1
            EventData::CreatedV1(ref data) => {
                aggregate.id = data.id;
                aggregate.created_at = self.timestamp;
                aggregate.updated_at = self.timestamp;
                aggregate.deleted_at = None;
                aggregate.version = 0;
                aggregate.email = data.email.clone();
                aggregate.token = data.token.clone();
                aggregate.trials = 0;
                aggregate.account_id = data.account_id;
            },
            // VerificationSucceededV1
            EventData::VerificationSucceededV1 => {},
            // VerificationFailedV1
            EventData::VerificationFailedV1(_) => {
                aggregate.trials = aggregate.trials + 1;
            },
            // PendingEmail
            EventData::DeletedV1 => {
                aggregate.deleted_at = Some(self.timestamp);
            },
        }
    }

    fn timestamp(&self) -> chrono::DateTime<chrono::Utc> {
        return self.timestamp;
    }
}
