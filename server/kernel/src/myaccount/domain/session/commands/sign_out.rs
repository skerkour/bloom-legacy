use crate::{error::KernelError, myaccount::domain::session};
use diesel::{
    r2d2::{ConnectionManager, PooledConnection},
    PgConnection,
};
use serde::{Deserialize, Serialize};
use eventsourcing::{Event, EventTs};


#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SignOut {}

impl eventsourcing::Command for SignOut {
    type Aggregate = session::Session;
    type Event = SignedOut;
    type Context = PooledConnection<ConnectionManager<PgConnection>>;
    type Error = KernelError;

    fn validate(
        &self,
        _ctx: &Self::Context,
        aggregate: &Self::Aggregate,
    ) -> Result<(), Self::Error> {
        if aggregate.deleted_at.is_some() {
            return Err(KernelError::Validation(
                "Session is currently not active.".to_string(),
            ));
        }
        return Ok(());
    }

    fn build_event(
        &self,
        _ctx: &Self::Context,
        aggregate: &Self::Aggregate,
    ) -> Result<Self::Event, Self::Error> {
        return Ok(SignedOut {
            timestamp: chrono::Utc::now(),
        });
    }
}

// Event
#[derive(Clone, Debug, Deserialize, EventTs, Serialize)]
pub struct SignedOut {
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

impl Event for SignedOut {
    type Aggregate = session::Session;

    fn apply(&self, aggregate: Self::Aggregate) -> Self::Aggregate {
        return Self::Aggregate {
            deleted_at: Some(self.timestamp),
            ..aggregate
        };
    }
}
