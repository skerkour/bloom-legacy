use crate::domain::contact;
use diesel::{
    r2d2::{ConnectionManager, PooledConnection},
    PgConnection,
};
use eventsourcing::{Event, EventTs};
use kernel::KernelError;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UpdateWebsites {
    pub websites: Vec<contact::Website>,
}

impl eventsourcing::Command for UpdateWebsites {
    type Aggregate = contact::Contact;
    type Event = WebsitesUpdated;
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
        return Ok(WebsitesUpdated {
            timestamp: chrono::Utc::now(),
            websites: self.websites.clone(),
        });
    }
}

// Event
#[derive(Clone, Debug, EventTs)]
pub struct WebsitesUpdated {
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub websites: Vec<contact::Website>,
}

impl Event for WebsitesUpdated {
    type Aggregate = contact::Contact;

    fn apply(&self, aggregate: Self::Aggregate) -> Self::Aggregate {
        return Self::Aggregate {
            websites: self.websites.clone(),
            ..aggregate
        };
    }
}
