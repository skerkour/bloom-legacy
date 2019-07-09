use crate::{domain::profile, DEFAULT_AVAILABLE_SPACE};
use diesel::{
    r2d2::{ConnectionManager, PooledConnection},
    PgConnection,
};
use eventsourcing::{Event, EventTs};
use kernel::KernelError;

#[derive(Clone, Debug)]
pub struct Create {
    pub account_id: uuid::Uuid,
    pub home_id: uuid::Uuid,
}

impl eventsourcing::Command for Create {
    type Aggregate = profile::Profile;
    type Event = Created;
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
        return Ok(Created {
            id: uuid::Uuid::new_v4(),
            timestamp: chrono::Utc::now(),
            home_id: self.home_id,
            total_space: DEFAULT_AVAILABLE_SPACE,
            account_id: self.account_id,
        });
    }
}

// Event
#[derive(Clone, Debug, EventTs)]
pub struct Created {
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub id: uuid::Uuid,
    pub home_id: uuid::Uuid,
    pub total_space: i64,
    pub account_id: uuid::Uuid,
}

impl Event for Created {
    type Aggregate = profile::Profile;

    fn apply(&self, _aggregate: Self::Aggregate) -> Self::Aggregate {
        return Self::Aggregate {
            id: self.id,
            created_at: self.timestamp,
            updated_at: self.timestamp,
            version: 0,
            total_space: self.total_space,
            used_space: 0,
            account_id: self.account_id,
            home_id: self.home_id,
        };
    }
}
