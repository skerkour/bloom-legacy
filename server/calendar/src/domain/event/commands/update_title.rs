use crate::domain::event;
use diesel::{
    r2d2::{ConnectionManager, PooledConnection},
    PgConnection,
};
use eventsourcing::{Event, EventTs};
use kernel::KernelError;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UpdateTitle {
    pub title: String,
}

impl eventsourcing::Command for UpdateTitle {
    type Aggregate = event::CalendarEvent;
    type Event = TitleUpdated;
    type Context = PooledConnection<ConnectionManager<PgConnection>>;
    type Error = KernelError;

    fn validate(
        &self,
        _ctx: &Self::Context,
        _aggregate: &Self::Aggregate,
    ) -> Result<(), Self::Error> {
        event::validators::title(&self.title)?;

        return Ok(());
    }

    fn build_event(
        &self,
        _ctx: &Self::Context,
        _aggregate: &Self::Aggregate,
    ) -> Result<Self::Event, Self::Error> {
        return Ok(TitleUpdated {
            timestamp: chrono::Utc::now(),
            title: self.title.clone(),
        });
    }
}

// Event
#[derive(Clone, Debug, Deserialize, EventTs, Serialize)]
pub struct TitleUpdated {
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub title: String,
}

impl Event for TitleUpdated {
    type Aggregate = event::CalendarEvent;

    fn apply(&self, aggregate: Self::Aggregate) -> Self::Aggregate {
        return Self::Aggregate {
            title: self.title.clone(),
            ..aggregate
        };
    }
}
