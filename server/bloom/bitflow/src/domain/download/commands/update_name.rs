use crate::domain::download;
use diesel::{
    r2d2::{ConnectionManager, PooledConnection},
    PgConnection,
};
use eventsourcing::{Event, EventTs};
use kernel::KernelError;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UpdateName {
    pub name: String,
}

impl eventsourcing::Command for UpdateName {
    type Aggregate = download::Download;
    type Event = NameUpdated;
    type Context = PooledConnection<ConnectionManager<PgConnection>>;
    type Error = KernelError;

    fn validate(
        &self,
        _ctx: &Self::Context,
        aggregate: &Self::Aggregate,
    ) -> Result<(), Self::Error> {
        if aggregate.removed_at.is_some() {
            return Err(KernelError::Validation(
                "Download has been removed".to_string(),
            ));
        }

        return Ok(());
    }

    fn build_event(
        &self,
        _ctx: &Self::Context,
        _aggregate: &Self::Aggregate,
    ) -> Result<Self::Event, Self::Error> {
        return Ok(NameUpdated {
            timestamp: chrono::Utc::now(),
            name: self.name.clone(),
        });
    }
}

// Event
#[derive(Clone, Debug, EventTs)]
pub struct NameUpdated {
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub name: String,
}

impl Event for NameUpdated {
    type Aggregate = download::Download;

    fn apply(&self, aggregate: Self::Aggregate) -> Self::Aggregate {
        return Self::Aggregate {
            name: self.name.clone(),
            ..aggregate
        };
    }
}
