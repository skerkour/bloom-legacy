use crate::domain::note;
use diesel::{
    r2d2::{ConnectionManager, PooledConnection},
    PgConnection,
};
use eventsourcing::{Event, EventTs};
use kernel::KernelError;

#[derive(Clone, Debug)]
pub struct Restore {}

impl eventsourcing::Command for Restore {
    type Aggregate = note::Note;
    type Event = Restored;
    type Context = PooledConnection<ConnectionManager<PgConnection>>;
    type Error = KernelError;

    fn validate(
        &self,
        _ctx: &Self::Context,
        aggregate: &Self::Aggregate,
    ) -> Result<(), Self::Error> {
        if aggregate.removed_at.is_none() {
            return Err(KernelError::NotFound("Note is not in trash".to_string()));
        }

        return Ok(());
    }

    fn build_event(
        &self,
        _ctx: &Self::Context,
        _aggregate: &Self::Aggregate,
    ) -> Result<Self::Event, Self::Error> {
        return Ok(Restored {
            timestamp: chrono::Utc::now(),
        });
    }
}

// Event
#[derive(Clone, Debug, EventTs)]
pub struct Restored {
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

impl Event for Restored {
    type Aggregate = note::Note;

    fn apply(&self, aggregate: Self::Aggregate) -> Self::Aggregate {
        return Self::Aggregate {
            removed_at: None,
            ..aggregate
        };
    }
}
