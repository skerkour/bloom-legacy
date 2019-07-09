use crate::domain::contact;
use diesel::{
    r2d2::{ConnectionManager, PooledConnection},
    PgConnection,
};
use eventsourcing::{Event, EventTs};
use kernel::KernelError;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UpdateLastName {
    pub last_name: Option<String>,
}

impl eventsourcing::Command for UpdateLastName {
    type Aggregate = contact::Contact;
    type Event = LastNameUpdated;
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
        return Ok(LastNameUpdated {
            timestamp: chrono::Utc::now(),
            last_name: self.last_name.clone(),
        });
    }
}

// Event
#[derive(Clone, Debug, EventTs)]
pub struct LastNameUpdated {
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub last_name: Option<String>,
}

impl Event for LastNameUpdated {
    type Aggregate = contact::Contact;

    fn apply(&self, aggregate: Self::Aggregate) -> Self::Aggregate {
        return Self::Aggregate {
            last_name: self.last_name.clone(),
            ..aggregate
        };
    }
}
