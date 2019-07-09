use crate::domain::note;
use diesel::{
    r2d2::{ConnectionManager, PooledConnection},
    PgConnection,
};
use eventsourcing::{Event, EventTs};
use kernel::KernelError;

#[derive(Clone, Debug)]
pub struct Unarchive {}

impl eventsourcing::Command for Unarchive {
    type Aggregate = note::Note;
    type Event = Unarchived;
    type Context = PooledConnection<ConnectionManager<PgConnection>>;
    type Error = KernelError;

    fn validate(
        &self,
        _ctx: &Self::Context,
        aggregate: &Self::Aggregate,
    ) -> Result<(), Self::Error> {
        if aggregate.archived_at.is_none() {
            return Err(KernelError::NotFound("Note is not in archive".to_string()));
        }

        return Ok(());
    }

    fn build_event(
        &self,
        _ctx: &Self::Context,
        _aggregate: &Self::Aggregate,
    ) -> Result<Self::Event, Self::Error> {
        return Ok(Unarchived {
            timestamp: chrono::Utc::now(),
        });
    }
}

// Event
#[derive(Clone, Debug, EventTs)]
pub struct Unarchived {
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

impl Event for Unarchived {
    type Aggregate = note::Note;

    fn apply(&self, aggregate: Self::Aggregate) -> Self::Aggregate {
        return Self::Aggregate {
            archived_at: None,
            ..aggregate
        };
    }
}
