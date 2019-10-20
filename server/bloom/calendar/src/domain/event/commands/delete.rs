use crate::domain::event;
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
    type Aggregate = event::CalendarEvent;
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
#[derive(Clone, Debug, Deserialize, EventTs, Serialize)]
pub struct Deleted {
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

impl Event for Deleted {
    type Aggregate = event::CalendarEvent;

    fn apply(&self, aggregate: Self::Aggregate) -> Self::Aggregate {
        return aggregate;
    }
}
