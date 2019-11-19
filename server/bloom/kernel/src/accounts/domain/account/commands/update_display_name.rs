use crate::{error::KernelError, accounts::domain::account};
use diesel::{
    r2d2::{ConnectionManager, PooledConnection},
    PgConnection,
};
use eventsourcing::{Event, EventTs};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug)]
pub struct UpdateDisplayName {
    pub display_name: String,
}

impl eventsourcing::Command for UpdateDisplayName {
    type Aggregate = account::Account;
    type Event = DisplayNameUpdated;
    type Context = PooledConnection<ConnectionManager<PgConnection>>;
    type Error = KernelError;

    fn validate(
        &self,
        _ctx: &Self::Context,
        _aggregate: &Self::Aggregate,
    ) -> Result<(), Self::Error> {
        account::validators::display_name(&self.display_name)?;

        return Ok(());
    }

    fn build_event(
        &self,
        _ctx: &Self::Context,
        _aggregate: &Self::Aggregate,
    ) -> Result<Self::Event, Self::Error> {
        return Ok(DisplayNameUpdated {
            timestamp: chrono::Utc::now(),
            display_name: self.display_name.clone(),
        });
    }
}

// Event
#[derive(Clone, Debug, Deserialize, EventTs, Serialize)]
pub struct DisplayNameUpdated {
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub display_name: String,
}

impl Event for DisplayNameUpdated {
    type Aggregate = account::Account;

    fn apply(&self, aggregate: Self::Aggregate) -> Self::Aggregate {
        return Self::Aggregate {
            display_name: self.display_name.clone(),
            ..aggregate
        };
    }
}
