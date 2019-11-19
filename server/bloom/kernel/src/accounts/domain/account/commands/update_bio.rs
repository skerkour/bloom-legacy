use crate::{error::KernelError, accounts::domain::account};
use diesel::{
    r2d2::{ConnectionManager, PooledConnection},
    PgConnection,
};
use eventsourcing::{Event, EventTs};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug)]
pub struct UpdateBio {
    pub bio: String,
}

impl eventsourcing::Command for UpdateBio {
    type Aggregate = account::Account;
    type Event = BioUpdated;
    type Context = PooledConnection<ConnectionManager<PgConnection>>;
    type Error = KernelError;

    fn validate(
        &self,
        _ctx: &Self::Context,
        _aggregate: &Self::Aggregate,
    ) -> Result<(), Self::Error> {
        account::validators::bio(&self.bio)?;

        return Ok(());
    }

    fn build_event(
        &self,
        _ctx: &Self::Context,
        _aggregate: &Self::Aggregate,
    ) -> Result<Self::Event, Self::Error> {
        return Ok(BioUpdated {
            timestamp: chrono::Utc::now(),
            bio: self.bio.clone(),
        });
    }
}

// Event
#[derive(Clone, Debug, Deserialize, EventTs, Serialize)]
pub struct BioUpdated {
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub bio: String,
}

impl Event for BioUpdated {
    type Aggregate = account::Account;

    fn apply(&self, aggregate: Self::Aggregate) -> Self::Aggregate {
        return Self::Aggregate {
            bio: self.bio.clone(),
            ..aggregate
        };
    }
}
