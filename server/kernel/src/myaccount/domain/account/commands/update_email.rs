use crate::{error::KernelError, myaccount::domain::account};
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
        // validators::email(self.config.disposable_email_domains(), &self.email)?;

        // verify that an email isn't already in use
        // already done in pending emial verify

        return Ok(());
    }

    fn build_event(
        &self,
        _ctx: &Self::Context,
        aggregate: &Self::Aggregate,
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
    type Aggregate = super::Account;

    fn apply(&self, aggregate: Self::Aggregate) -> Self::Aggregate {
        return Self::Aggregate {
            email: self.email.clone(),
            ..aggregate
        };
    }
}
