use crate::domain::file;
use diesel::{
    r2d2::{ConnectionManager, PooledConnection},
    PgConnection,
};
use kernel::KernelError;

#[derive(Clone, Debug)]
pub struct Delete {}

impl eventsourcing::Command for Delete {
    type Aggregate = file::File;
    type Event = Deleted;
    type Context = PooledConnection<ConnectionManager<PgConnection>>;
    type Error = KernelError;

    fn validate(
        &self,
        _ctx: &Self::Context,
        aggregate: &Self::Aggregate,
    ) -> Result<(), Self::Error> {
        if aggregate.deleted_at.is_some() {
            return Err(KernelError::Validation(
                "File has already been deleted".to_string(),
            ));
        }

        return Ok(());
    }

    fn build_event(
        &self,
        _ctx: &Self::Context,
        aggregate: &Self::Aggregate,
    ) -> Result<Self::Event, Self::Error> {
        return Ok(Deleted {
            timestamp: chrono::Utc::now(),
        });
    }
}

// Event
#[derive(Clone, Debug, EventTs)]
pub struct Deleted {
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

impl Event for Deleted {
    type Aggregate = file::File;

    fn apply(&self, aggregate: Self::Aggregate) -> Self::Aggregate {
        return Self::Aggregate {
            explicitly_trashed: false,
            trashed_at: None,
            deleted_at: Some(self.timestamp),
            ..aggregate
        };
    }
}
