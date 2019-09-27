use crate::error::KernelError;
use crate::myaccount::domain::pending_account;
use chrono::Utc;
use diesel::{
    r2d2::{ConnectionManager, PooledConnection},
    PgConnection,
};
use eventsourcing::{Event, EventTs};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Verify {
    pub id: uuid::Uuid,
    pub code: String,
}

impl eventsourcing::Command for Verify {
    type Aggregate = pending_account::PendingAccount;
    type Event = Verified;
    type Context = PooledConnection<ConnectionManager<PgConnection>>;
    type Error = KernelError;

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
    ) -> Result<Self::Event, Self::Error> {
        let timestamp = Utc::now();
        let duration = aggregate.created_at.signed_duration_since(timestamp);

        if aggregate.trials + 1 >= 10 {
            return Err(KernelError::Validation(
                pending_account::VerificationFailedReason::TooManyTrials.to_string(),
            ));
        } else if !bcrypt::verify(&self.code, &aggregate.verification_code_hash)
            .map_err(|_| KernelError::Bcrypt)? {
            // verify given code
            return Err(KernelError::Validation(
                pending_account::VerificationFailedReason::CodeNotValid.to_string(),
            ));
        } else if duration.num_minutes() > 30 {
            // verify code expiration
            return Err(KernelError::Validation(
                pending_account::VerificationFailedReason::CodeExpired.to_string(),
            ));
        }

        return Ok(Verified { timestamp });
    }
}

// Event
#[derive(Clone, Debug, Deserialize, EventTs, Serialize)]
pub struct Verified {
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

impl Event for Verified {
    type Aggregate = pending_account::PendingAccount;

    fn apply(&self, aggregate: Self::Aggregate) -> Self::Aggregate {
        return Self::Aggregate {
            verified: true,
            ..aggregate
        };
    }
}
