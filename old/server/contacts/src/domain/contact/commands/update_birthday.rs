use crate::domain::contact;
use diesel::{
    r2d2::{ConnectionManager, PooledConnection},
    PgConnection,
};
use eventsourcing::{Event, EventTs};
use kernel::KernelError;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UpdateBirthday {
    pub birthday: Option<chrono::DateTime<chrono::Utc>>,
}

impl eventsourcing::Command for UpdateBirthday {
    type Aggregate = contact::Contact;
    type Event = BirthdayUpdated;
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
        return Ok(BirthdayUpdated {
            timestamp: chrono::Utc::now(),
            birthday: self.birthday,
        });
    }
}

// Event
#[derive(Clone, Debug, EventTs)]
pub struct BirthdayUpdated {
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub birthday: Option<chrono::DateTime<chrono::Utc>>,
}

impl Event for BirthdayUpdated {
    type Aggregate = contact::Contact;

    fn apply(&self, aggregate: Self::Aggregate) -> Self::Aggregate {
        return Self::Aggregate {
            birthday: self.birthday,
            ..aggregate
        };
    }
}
