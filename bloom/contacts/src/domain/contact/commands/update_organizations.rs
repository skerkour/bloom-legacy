use crate::domain::contact;
use diesel::{
    r2d2::{ConnectionManager, PooledConnection},
    PgConnection,
};
use eventsourcing::{Event, EventTs};
use kernel::KernelError;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UpdateOrganizations {
    pub organizations: Vec<contact::Organization>,
}

impl eventsourcing::Command for UpdateOrganizations {
    type Aggregate = contact::Contact;
    type Event = OrganizationsUpdated;
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
        return Ok(OrganizationsUpdated {
            timestamp: chrono::Utc::now(),
            organizations: self.organizations.clone(),
        });
    }
}

// Event
#[derive(Clone, Debug, EventTs)]
pub struct OrganizationsUpdated {
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub organizations: Vec<contact::Organization>,
}

impl Event for OrganizationsUpdated {
    type Aggregate = contact::Contact;

    fn apply(&self, aggregate: Self::Aggregate) -> Self::Aggregate {
        return Self::Aggregate {
            organizations: self.organizations.clone(),
            ..aggregate
        };
    }
}
