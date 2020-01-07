use crate::domain::contact;
use diesel::{
    r2d2::{ConnectionManager, PooledConnection},
    PgConnection,
};
use eventsourcing::{Event, EventTs};
use kernel::KernelError;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UpdateAddresses {
    pub addresses: Vec<contact::Address>,
}

impl eventsourcing::Command for UpdateAddresses {
    type Aggregate = contact::Contact;
    type Event = AddressesUpdated;
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
        return Ok(AddressesUpdated {
            timestamp: chrono::Utc::now(),
            addresses: self.addresses.clone(),
        });
    }
}

// Event
#[derive(Clone, Debug, EventTs)]
pub struct AddressesUpdated {
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub addresses: Vec<contact::Address>,
}

impl Event for AddressesUpdated {
    type Aggregate = contact::Contact;

    fn apply(&self, aggregate: Self::Aggregate) -> Self::Aggregate {
        return Self::Aggregate {
            addresses: self.addresses.clone(),
            ..aggregate
        };
    }
}
