use crate::accounts::domain::session;
use bloom_error::BloomError;
use diesel::{
    r2d2::{ConnectionManager, PooledConnection},
    PgConnection,
};
use eventsourcing::{Event, EventTs};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SignOut {}

impl eventsourcing::Command for SignOut {
    type Aggregate = session::Session;
    type Event = SignedOut;
    type Context = PooledConnection<ConnectionManager<PgConnection>>;
    type Error = BloomError;

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
        return aggregate;
    }
}
