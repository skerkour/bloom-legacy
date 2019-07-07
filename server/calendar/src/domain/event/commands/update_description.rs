use crate::{domain::event, validators};
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
        aggregate: &Self::Aggregate,
    ) -> Result<(), Self::Error> {
        if aggregate.deleted_at.is_some() {
            return Err(KernelError::NotFound("Event not found".to_string()));
        }

        validators::event_description(&self.description)?;

        return Ok(());
    }

    fn build_event(
        &self,
        _ctx: &Self::Context,
        aggregate: &Self::Aggregate,
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
    type Aggregate = CalendarEvent;

    fn apply(&self, _aggregate: Self::Aggregate) -> Self::Aggregate {
        return Self::Aggregate {
            deleted_at: Some(self.timestamp),
            ..aggregate
        };
    }
}
