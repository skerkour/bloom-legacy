#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreatedV1 {
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub id: uuid::Uuid,
    pub email: String,
    pub token: String,
    pub account_id: uuid::Uuid,
    pub code: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Deleted {
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct VerificationFailed {
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub reason: VerificationFailedReason,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Verified {
    pub timestamp: chrono::DateTime<chrono::Utc>,
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
            }
            // VerificationFailedV1
            EventData::VerificationFailedV1(_) => {
                aggregate.trials = aggregate.trials + 1;
            }
            // PendingEmail
            EventData::DeletedV1 => {
                aggregate.deleted_at = Some(self.timestamp);
            }
        }
    }

    fn timestamp(&self) -> chrono::DateTime<chrono::Utc> {
        return self.timestamp;
    }
}
