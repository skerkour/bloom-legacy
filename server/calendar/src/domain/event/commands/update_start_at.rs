use crate::domain::event;
use diesel::{
    r2d2::{ConnectionManager, PooledConnection},
    PgConnection,
};
use eventsourcing::{Event, EventTs};
use kernel::KernelError;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UpdateStartAt {
    pub start_at: chrono::DateTime<chrono::Utc>,
}

impl eventsourcing::Command for UpdateStartAt {
    type Aggregate = event::CalendarEvent;
    type Event = StartAtUpdated;
    type Context = PooledConnection<ConnectionManager<PgConnection>>;
    type Error = KernelError;

    fn validate(
        &self,
        _ctx: &Self::Context,
        aggregate: &Self::Aggregate,
    ) -> Result<(), Self::Error> {
        event::validators::dates(self.start_at, aggregate.end_at)?;

        return Ok(());
    }

    fn build_event(
        &self,
        _ctx: &Self::Context,
        _aggregate: &Self::Aggregate,
    ) -> Result<Self::Event, Self::Error> {
        return Ok(StartAtUpdated {
            timestamp: chrono::Utc::now(),
            start_at: self.start_at,
        });
    }
}

// Event
#[derive(Clone, Debug, Deserialize, EventTs, Serialize)]
pub struct StartAtUpdated {
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub start_at: chrono::DateTime<chrono::Utc>,
}

impl Event for StartAtUpdated {
    type Aggregate = event::CalendarEvent;

    fn apply(&self, aggregate: Self::Aggregate) -> Self::Aggregate {
        return Self::Aggregate {
            start_at: self.start_at,
            ..aggregate
        };
    }
}
