use crate::{error::KernelError, accounts::domain::account};
use diesel::{
    r2d2::{ConnectionManager, PooledConnection},
    PgConnection,
};
use eventsourcing::{Event, EventTs};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug)]
pub struct UpdateLastName {
    pub last_name: String,
}

impl eventsourcing::Command for UpdateLastName {
    type Aggregate = account::Account;
    type Event = account::LastNameUpdated;
    type Context = PooledConnection<ConnectionManager<PgConnection>>;
    type Error = KernelError;

    fn validate(
        &self,
        _ctx: &Self::Context,
        _aggregate: &Self::Aggregate,
    ) -> Result<(), Self::Error> {
        account::validators::last_name(&self.last_name)?;

        return Ok(());
    }

    fn build_event(
        &self,
        _ctx: &Self::Context,
        _aggregate: &Self::Aggregate,
    ) -> Result<Self::Event, Self::Error> {
        return Ok(account::LastNameUpdated {
            timestamp: chrono::Utc::now(),
            last_name: self.last_name.clone(),
        });
    }
}

// Event
#[derive(Clone, Debug, Deserialize, EventTs, Serialize)]
pub struct LastNameUpdated {
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub last_name: String,
}

impl Event for LastNameUpdated {
    type Aggregate = account::Account;

    fn apply(&self, aggregate: Self::Aggregate) -> Self::Aggregate {
        return Self::Aggregate {
            last_name: self.last_name.clone(),
            ..aggregate
        };
    }
}
