use crate::events::EventMetadata;
use serde::{Deserialize, Serialize};

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
    NewCodeSentV1(NewCodeSentV1),
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
                "Code has expired. Please create another account.".to_string()
            }
            VerificationFailedReason::TooManyTrials => {
                "Maximum number of trials reached. Please create another account.".to_string()
            }
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct NewCodeSentV1 {
    pub token_hash: String,
    pub code: String,
}

impl eventsourcing::Event for Event {
    type Aggregate = super::PendingAccount;

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
                aggregate.first_name = data.first_name.clone();
                aggregate.last_name = data.last_name.clone();
                aggregate.password = data.password.clone();
                aggregate.token = data.token.clone();
                aggregate.trials = 0;
                aggregate.verified = false;
            }
            // VerificationSucceededV1
            EventData::VerificationSucceededV1 => {
                aggregate.verified = true;
            }
            // VerificationFailedV1
            EventData::VerificationFailedV1(_) => {
                aggregate.trials = aggregate.trials + 1;
            }
            // RegistrationCompletedV1
            EventData::RegistrationCompletedV1 => {
                aggregate.deleted_at = Some(self.timestamp);
            }
            // NewCodeSentV1
            EventData::NewCodeSentV1(ref data) => {
                aggregate.token = data.token_hash.clone();
            }
        }
    }

    fn timestamp(&self) -> chrono::DateTime<chrono::Utc> {
        return self.timestamp;
    }
}
