use crate::domain::contact;
use diesel::{
    r2d2::{ConnectionManager, PooledConnection},
    PgConnection,
};
use eventsourcing::{Event, EventTs};
use kernel::KernelError;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UpdateEmails {
    pub emails: Vec<contact::Email>,
}

impl eventsourcing::Command for UpdateEmails {
    type Aggregate = contact::Contact;
    type Event = EmailsUpdated;
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
        return Ok(EmailsUpdated {
            timestamp: chrono::Utc::now(),
            emails: self.emails.clone(),
        });
    }
}

// Event
#[derive(Clone, Debug, EventTs)]
pub struct EmailsUpdated {
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub emails: Vec<contact::Email>,
}

impl Event for EmailsUpdated {
    type Aggregate = contact::Contact;

    fn apply(&self, aggregate: Self::Aggregate) -> Self::Aggregate {
        return Self::Aggregate {
            emails: self.emails.clone(),
            ..aggregate
        };
    }
}
