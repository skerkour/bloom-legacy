use crate::{error::KernelError, myaccount::domain::deleted_username};
use diesel::{
    r2d2::{ConnectionManager, PooledConnection},
    PgConnection,
};
use eventsourcing::{Event, EventTs};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug)]
pub struct Create {
    pub username: String,
}

impl eventsourcing::Command for Create {
    type Aggregate = deleted_username::DeletedUsername;
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
            timestamp: chrono::Utc::now(),
            username: self.username.clone(),
        });
    }
}

// Event
#[derive(Clone, Debug, Deserialize, EventTs, Serialize)]
pub struct Created {
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub username: String,
}

impl Event for Created {
    type Aggregate = deleted_username::DeletedUsername;

    fn apply(&self, _aggregate: Self::Aggregate) -> Self::Aggregate {
        return Self::Aggregate {
            created_at: self.timestamp,
            username: self.username.clone(),
            version: 0,
            updated_at: self.timestamp,
        };
    }
}
