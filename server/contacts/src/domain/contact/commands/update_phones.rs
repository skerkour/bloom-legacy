use crate::domain::contact;
use diesel::{
    r2d2::{ConnectionManager, PooledConnection},
    PgConnection,
};
use eventsourcing::{Event, EventTs};
use kernel::KernelError;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UpdatePhones {
    pub phones: Vec<contact::Phone>,
}

impl eventsourcing::Command for UpdatePhones {
    type Aggregate = contact::Contact;
    type Event = PhonesUpdated;
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
        return Ok(PhonesUpdated {
            timestamp: chrono::Utc::now(),
            phones: self.phones.clone(),
        });
    }
}

// Event
#[derive(Clone, Debug, EventTs)]
pub struct PhonesUpdated {
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub phones: Vec<contact::Phone>,
}

impl Event for PhonesUpdated {
    type Aggregate = contact::Contact;

    fn apply(&self, aggregate: Self::Aggregate) -> Self::Aggregate {
        return Self::Aggregate {
            phones: self.phones.clone(),
            ..aggregate
        };
    }
}
