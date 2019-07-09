use crate::domain::event;
use diesel::{
    r2d2::{ConnectionManager, PooledConnection},
    PgConnection,
};
use eventsourcing::{Event, EventTs};
use kernel::KernelError;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UpdateDescription {
    pub description: String,
}

impl eventsourcing::Command for UpdateDescription {
    type Aggregate = event::CalendarEvent;
    type Event = DescriptionUpdated;
    type Context = PooledConnection<ConnectionManager<PgConnection>>;
    type Error = KernelError;

    fn validate(
        &self,
        _ctx: &Self::Context,
        _aggregate: &Self::Aggregate,
    ) -> Result<(), Self::Error> {
        event::validators::description(&self.description)?;

        return Ok(());
    }

    fn build_event(
        &self,
        _ctx: &Self::Context,
        _aggregate: &Self::Aggregate,
    ) -> Result<Self::Event, Self::Error> {
        return Ok(DescriptionUpdated {
            timestamp: chrono::Utc::now(),
            description: self.description.clone(),
        });
    }
}

// Event
#[derive(Clone, Debug, Deserialize, EventTs, Serialize)]
pub struct DescriptionUpdated {
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub description: String,
}

impl Event for DescriptionUpdated {
    type Aggregate = event::CalendarEvent;

    fn apply(&self, aggregate: Self::Aggregate) -> Self::Aggregate {
        return Self::Aggregate {
            description: self.description.clone(),
            ..aggregate
        };
    }
}
