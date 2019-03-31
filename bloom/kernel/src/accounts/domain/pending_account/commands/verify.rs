use crate::{
    events::EventMetadata,
    accounts::domain::pending_accounts,
};
use crate::error::KernelError;
use serde::{Serialize, Deserialize};
use chrono::Utc;
use diesel::{
    PgConnection,
    r2d2::{PooledConnection, ConnectionManager},
};


#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Verify {
    pub id: uuid::Uuid,
    pub code: String,
    pub metadata: EventMetadata,
}


impl<'a> eventsourcing::Command<'a> for Verify {
    type Aggregate = pending_accounts::PendingAccount;
    type Event = pending_accounts::Event;
    type Context = PooledConnection<ConnectionManager<PgConnection>>;
    type Error = KernelError;
    type NonStoredData = ();

    fn validate(&self, _ctx: &Self::Context, _aggregate: &Self::Aggregate) -> Result<(), Self::Error> {
        return Ok(());
    }

    fn build_event(&self, _ctx: &Self::Context, aggregate: &Self::Aggregate) -> Result<(Self::Event, Self::NonStoredData), Self::Error> {
        let metadata = self.metadata.clone();
        let timestamp = Utc::now();
        let duration = aggregate.created_at.signed_duration_since(timestamp);

        let data = if aggregate.trials + 1 >= 10 {
            pending_accounts::EventData::VerificationFailedV1(pending_accounts::VerificationFailedReason::TooManyTrials)
        } else if !bcrypt::verify(&self.code, &aggregate.token).map_err(|_| KernelError::Bcrypt)? {
            // verify given code
            pending_accounts::EventData::VerificationFailedV1(pending_accounts::VerificationFailedReason::CodeNotValid)
        } else if duration.num_minutes() > 30 {
            // verify code expiration
            pending_accounts::EventData::VerificationFailedV1(pending_accounts::VerificationFailedReason::CodeExpired)
        } else {
            pending_accounts::EventData::VerificationSucceededV1
        };

        return  Ok((pending_accounts::Event{
            id: uuid::Uuid::new_v4(),
            timestamp,
            data,
            aggregate_id: aggregate.id,
            metadata,
        }, ()));
    }
}

