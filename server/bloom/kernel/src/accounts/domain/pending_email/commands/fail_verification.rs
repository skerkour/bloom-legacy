use crate::{error::KernelError, accounts::domain::pending_email};
use diesel::{
    r2d2::{ConnectionManager, PooledConnection},
    PgConnection,
};
use eventsourcing::{Event, EventTs};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct FailVerification {}

impl eventsourcing::Command for FailVerification {
    type Aggregate = pending_email::PendingEmail;
    type Event = VerificationFailed;
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
        _aggregate: &Self::Aggregate,
    ) -> Result<Self::Event, Self::Error> {
        return Ok(VerificationFailed {
            timestamp: chrono::Utc::now(),
        });
    }
}

// Event
#[derive(Clone, Debug, Deserialize, EventTs, Serialize)]
pub struct VerificationFailed {
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

impl Event for VerificationFailed {
    type Aggregate = pending_email::PendingEmail;

    fn apply(&self, aggregate: Self::Aggregate) -> Self::Aggregate {
        return Self::Aggregate {
            trials: aggregate.trials + 1,
            ..aggregate
        };
    }
}
