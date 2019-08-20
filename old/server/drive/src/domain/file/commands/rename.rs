use crate::domain::file;
use diesel::{
    r2d2::{ConnectionManager, PooledConnection},
    PgConnection,
};
use eventsourcing::{Event, EventTs};
use kernel::KernelError;

#[derive(Clone, Debug)]
pub struct Rename {
    pub name: String,
}

impl eventsourcing::Command for Rename {
    type Aggregate = file::File;
    type Event = Renamed;
    type Context = PooledConnection<ConnectionManager<PgConnection>>;
    type Error = KernelError;

    fn validate(
        &self,
        _ctx: &Self::Context,
        aggregate: &Self::Aggregate,
    ) -> Result<(), Self::Error> {
        if aggregate.trashed_at.is_some() {
            return Err(KernelError::Validation("File is in trash".to_string()));
        }
        file::validators::name(&self.name)?;

        return Ok(());
    }

    fn build_event(
        &self,
        _ctx: &Self::Context,
        _aggregate: &Self::Aggregate,
    ) -> Result<Self::Event, Self::Error> {
        return Ok(Renamed {
            timestamp: chrono::Utc::now(),
            name: self.name.clone(),
        });
    }
}

// Event
#[derive(Clone, Debug, EventTs)]
pub struct Renamed {
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub name: String,
}

impl Event for Renamed {
    type Aggregate = file::File;

    fn apply(&self, aggregate: Self::Aggregate) -> Self::Aggregate {
        return Self::Aggregate {
            name: self.name.clone(),
            ..aggregate
        };
    }
}
