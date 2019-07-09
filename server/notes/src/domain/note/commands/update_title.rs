use crate::domain::note;
use diesel::{
    r2d2::{ConnectionManager, PooledConnection},
    PgConnection,
};
use eventsourcing::{Event, EventTs};
use kernel::KernelError;

#[derive(Clone, Debug)]
pub struct UpdateTitle {
    pub title: String,
}

impl eventsourcing::Command for UpdateTitle {
    type Aggregate = note::Note;
    type Event = TitleUpdated;
    type Context = PooledConnection<ConnectionManager<PgConnection>>;
    type Error = KernelError;

    fn validate(
        &self,
        _ctx: &Self::Context,
        _aggregate: &Self::Aggregate,
    ) -> Result<(), Self::Error> {
        note::validators::title(&self.title)?;

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
#[derive(Clone, Debug, EventTs)]
pub struct TitleUpdated {
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub title: String,
}

impl Event for TitleUpdated {
    type Aggregate = note::Note;

    fn apply(&self, aggregate: Self::Aggregate) -> Self::Aggregate {
        return Self::Aggregate {
            title: self.title.clone(),
            ..aggregate
        };
    }
}
