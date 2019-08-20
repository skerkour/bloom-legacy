use crate::domain::album;
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
    type Aggregate = album::Album;
    type Event = Renamed;
    type Context = PooledConnection<ConnectionManager<PgConnection>>;
    type Error = KernelError;

    fn validate(
        &self,
        _ctx: &Self::Context,
        _aggregate: &Self::Aggregate,
    ) -> Result<(), Self::Error> {
        album::validators::name(&self.name)?;

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
    type Aggregate = album::Album;

    fn apply(&self, aggregate: Self::Aggregate) -> Self::Aggregate {
        return Self::Aggregate {
            name: self.name.clone(),
            ..aggregate
        };
    }
}
