use crate::error::KernelError;
use crate::{events::EventMetadata, myaccount::domain::pending_account};
use chrono::Utc;
use diesel::{
    r2d2::{ConnectionManager, PooledConnection},
    PgConnection,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Verify {
    pub id: uuid::Uuid,
    pub code: String,
    pub metadata: EventMetadata,
}

impl eventsourcing::Command for Verify {
    type Aggregate = pending_account::PendingAccount;
    type Event = pending_account::Event;
    type Context = PooledConnection<ConnectionManager<PgConnection>>;
    type Error = KernelError;
    type NonStoredData = ();

    fn validate(
        &self,
        _ctx: &Self::Context,
        _aggregate: &Self::Aggregate,
    ) -> Result<(), Self::Error> {
        return Ok(());
    }

    fn build_event(
        &self,
        _ctx: &Self::Context,
        aggregate: &Self::Aggregate,
    ) -> Result<(Self::Event, Self::NonStoredData), Self::Error> {
        let metadata = self.metadata.clone();
        let timestamp = Utc::now();
        let duration = aggregate.created_at.signed_duration_since(timestamp);

        let data = if aggregate.trials + 1 >= 10 {
            pending_account::EventData::VerificationFailedV1(
                pending_account::VerificationFailedReason::TooManyTrials,
            )
        } else if !bcrypt::verify(&self.code, &aggregate.token).map_err(|_| KernelError::Bcrypt)? {
            // verify given code
            pending_account::EventData::VerificationFailedV1(
                pending_account::VerificationFailedReason::CodeNotValid,
            )
        } else if duration.num_minutes() > 30 {
            // verify code expiration
            pending_account::EventData::VerificationFailedV1(
                pending_account::VerificationFailedReason::CodeExpired,
            )
        } else {
            pending_account::EventData::VerificationSucceededV1
        };

        return Ok((
            pending_account::Event {
                id: uuid::Uuid::new_v4(),
                timestamp,
                data,
                aggregate_id: aggregate.id,
                metadata,
            },
            (),
        ));
    }
}
