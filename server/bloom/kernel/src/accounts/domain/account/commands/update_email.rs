use crate::{error::KernelError, accounts::domain::account};
use diesel::{
    r2d2::{ConnectionManager, PooledConnection},
    PgConnection,
};
use eventsourcing::{Event, EventTs};

#[derive(Clone, Debug)]
pub struct UpdateEmail {
    pub email: String,
}

impl eventsourcing::Command for UpdateEmail {
    type Aggregate = account::Account;
    type Event = EmailUpdated;
    type Context = PooledConnection<ConnectionManager<PgConnection>>;
    type Error = KernelError;

    fn validate(
        &self,
        _ctx: &Self::Context,
        _aggregate: &Self::Aggregate,
    ) -> Result<(), Self::Error> {
        // validations must be done in pending_email::verify

        return Ok(());
    }

    fn build_event(
        &self,
        _ctx: &Self::Context,
        _aggregate: &Self::Aggregate,
    ) -> Result<Self::Event, Self::Error> {
        return Ok(EmailUpdated {
            timestamp: chrono::Utc::now(),
            email: self.email.clone(),
        });
    }
}

// Event
#[derive(Clone, Debug, EventTs)]
pub struct EmailUpdated {
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub email: String,
}

impl Event for EmailUpdated {
    type Aggregate = account::Account;

    fn apply(&self, aggregate: Self::Aggregate) -> Self::Aggregate {
        return Self::Aggregate {
            email: self.email.clone(),
            ..aggregate
        };
    }
}
