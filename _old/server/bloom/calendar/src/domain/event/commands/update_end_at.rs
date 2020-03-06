use crate::domain::event;
use diesel::{
    r2d2::{ConnectionManager, PooledConnection},
    PgConnection,
};
use eventsourcing::{Event, EventTs};
use kernel::KernelError;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UpdateEndAt {
    pub end_at: chrono::DateTime<chrono::Utc>,
}

impl eventsourcing::Command for UpdateEndAt {
    type Aggregate = event::CalendarEvent;
    type Event = EndAtUpdated;
    type Context = PooledConnection<ConnectionManager<PgConnection>>;
    type Error = KernelError;

    fn validate(
        &self,
        _ctx: &Self::Context,
        aggregate: &Self::Aggregate,
    ) -> Result<(), Self::Error> {
        event::validators::dates(aggregate.start_at, self.end_at)?;

        return Ok(());
    }

    fn build_event(
        &self,
        _ctx: &Self::Context,
        _aggregate: &Self::Aggregate,
    ) -> Result<Self::Event, Self::Error> {
        return Ok(EndAtUpdated {
            timestamp: chrono::Utc::now(),
            end_at: self.end_at,
        });
    }
}

// Event
#[derive(Clone, Debug, Deserialize, EventTs, Serialize)]
pub struct EndAtUpdated {
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub end_at: chrono::DateTime<chrono::Utc>,
}

impl Event for EndAtUpdated {
    type Aggregate = event::CalendarEvent;

    fn apply(&self, aggregate: Self::Aggregate) -> Self::Aggregate {
        return Self::Aggregate {
            end_at: self.end_at,
            ..aggregate
        };
    }
}
