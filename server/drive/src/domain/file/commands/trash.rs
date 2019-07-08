use crate::domain::file;
use diesel::{
    r2d2::{ConnectionManager, PooledConnection},
    PgConnection,
};
use eventsourcing::{Event, EventTs};
use kernel::KernelError;

#[derive(Clone, Debug)]
pub struct Trash {
    pub explicitly_trashed: bool,
}

impl eventsourcing::Command for Trash {
    type Aggregate = file::File;
    type Event = Trashed;
    type Context = PooledConnection<ConnectionManager<PgConnection>>;
    type Error = KernelError;

    fn validate(
        &self,
        _ctx: &Self::Context,
        aggregate: &Self::Aggregate,
    ) -> Result<(), Self::Error> {
        if aggregate.trashed_at.is_some() {
            return Err(KernelError::Validation(
                "File is already in trash".to_string(),
            ));
        }

        return Ok(());
    }

    fn build_event(
        &self,
        _ctx: &Self::Context,
        _aggregate: &Self::Aggregate,
    ) -> Result<Self::Event, Self::Error> {
        return Ok(Trashed {
            timestamp: chrono::Utc::now(),
            explicitly_trashed: self.explicitly_trashed,
        });
    }
}

// Event
#[derive(Clone, Debug, EventTs)]
pub struct Trashed {
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub explicitly_trashed: bool,
}

impl Event for Trashed {
    type Aggregate = file::File;

    fn apply(&self, aggregate: Self::Aggregate) -> Self::Aggregate {
        return Self::Aggregate {
            explicitly_trashed: self.explicitly_trashed,
            trashed_at: Some(self.timestamp),
            ..aggregate
        };
    }
}
