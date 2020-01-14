use crate::domain::playlist;
use diesel::{
    r2d2::{ConnectionManager, PooledConnection},
    PgConnection,
};
use eventsourcing::{Event, EventTs};
use kernel::KernelError;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Delete {}

impl eventsourcing::Command for Delete {
    type Aggregate = playlist::Playlist;
    type Event = Deleted;
    type Context = PooledConnection<ConnectionManager<PgConnection>>;
    type Error = KernelError;

    fn validate(
        &self,
        _ctx: &Self::Context,
        _aggregate: &Self::Aggregate,
    ) -> Result<(), Self::Error> {
        return Ok(());
    }

    // TODO: improve
    fn build_event(
        &self,
        _ctx: &Self::Context,
        _aggregate: &Self::Aggregate,
    ) -> Result<Self::Event, Self::Error> {
        return Ok(Deleted {
            timestamp: chrono::Utc::now(),
        });
    }
}

// Event
#[derive(Clone, Debug, EventTs)]
pub struct Deleted {
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

impl Event for Deleted {
    type Aggregate = playlist::Playlist;

    fn apply(&self, aggregate: Self::Aggregate) -> Self::Aggregate {
        return aggregate;
    }
}
