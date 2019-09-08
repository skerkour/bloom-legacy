use crate::domain::contact;
use diesel::{
    r2d2::{ConnectionManager, PooledConnection},
    PgConnection,
};
use eventsourcing::{Event, EventTs};
use kernel::KernelError;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UpdateFirstName {
    pub first_name: Option<String>,
}

impl eventsourcing::Command for UpdateFirstName {
    type Aggregate = contact::Contact;
    type Event = FirstNameUpdated;
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
        return Ok(FirstNameUpdated {
            timestamp: chrono::Utc::now(),
            first_name: self.first_name.clone(),
        });
    }
}

// Event
#[derive(Clone, Debug, EventTs)]
pub struct FirstNameUpdated {
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub first_name: Option<String>,
}

impl Event for FirstNameUpdated {
    type Aggregate = contact::Contact;

    fn apply(&self, aggregate: Self::Aggregate) -> Self::Aggregate {
        return Self::Aggregate {
            first_name: self.first_name.clone(),
            ..aggregate
        };
    }
}
