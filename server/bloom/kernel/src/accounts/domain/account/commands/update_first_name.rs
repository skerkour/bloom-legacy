use crate::{error::KernelError, accounts::domain::account};
use diesel::{
    r2d2::{ConnectionManager, PooledConnection},
    PgConnection,
};
use eventsourcing::{Event, EventTs};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug)]
pub struct UpdateFirstName {
    pub first_name: String,
}

impl eventsourcing::Command for UpdateFirstName {
    type Aggregate = account::Account;
    type Event = FirstNameUpdated;
    type Context = PooledConnection<ConnectionManager<PgConnection>>;
    type Error = KernelError;

    fn validate(
        &self,
        _ctx: &Self::Context,
        _aggregate: &Self::Aggregate,
    ) -> Result<(), Self::Error> {
        account::validators::first_name(&self.first_name)?;

        return Ok(());
    }

    fn build_event(
        &self,
        _ctx: &Self::Context,
        _aggregate: &Self::Aggregate,
    ) -> Result<Self::Event, Self::Error> {
        return Ok(FirstNameUpdated {
            timestamp: chrono::Utc::now(),
            first_name: self.first_name.clone(),
        });
    }
}

// Event
#[derive(Clone, Debug, Deserialize, EventTs, Serialize)]
pub struct FirstNameUpdated {
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub first_name: String,
}

impl Event for FirstNameUpdated {
    type Aggregate = account::Account;

    fn apply(&self, aggregate: Self::Aggregate) -> Self::Aggregate {
        return Self::Aggregate {
            first_name: self.first_name.clone(),
            ..aggregate
        };
    }
}
