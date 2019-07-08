use crate::domain::profile;
use diesel::{
    r2d2::{ConnectionManager, PooledConnection},
    PgConnection,
};
use eventsourcing::{Event, EventTs};
use kernel::KernelError;

#[derive(Clone, Debug)]
pub struct UpdateUsedSpace {
    pub space: i64,
}

impl eventsourcing::Command for UpdateUsedSpace {
    type Aggregate = profile::Profile;
    type Event = UsedSpaceUpdated;
    type Context = PooledConnection<ConnectionManager<PgConnection>>;
    type Error = KernelError;

    fn validate(
        &self,
        _ctx: &Self::Context,
        _aggregate: &Self::Aggregate,
    ) -> Result<(), Self::Error> {
        return Ok(());
    }

    fn build_event(
        &self,
        _ctx: &Self::Context,
        _aggregate: &Self::Aggregate,
    ) -> Result<Self::Event, Self::Error> {
        return Ok(UsedSpaceUpdated {
            timestamp: chrono::Utc::now(),
            space: self.space,
        });
    }
}

// Event
#[derive(Clone, Debug, EventTs)]
pub struct UsedSpaceUpdated {
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub space: i64,
}

impl Event for UsedSpaceUpdated {
    type Aggregate = profile::Profile;

    fn apply(&self, aggregate: Self::Aggregate) -> Self::Aggregate {
        return Self::Aggregate {
            used_space: aggregate.used_space + self.space,
            ..aggregate
        };
    }
}
