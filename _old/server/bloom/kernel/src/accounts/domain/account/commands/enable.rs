use crate::{error::KernelError, accounts::domain::account};
use diesel::{
    r2d2::{ConnectionManager, PooledConnection},
    PgConnection,
};
use eventsourcing::{Event, EventTs};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug)]
pub struct Enable {
    pub actor: account::Account,
}

impl eventsourcing::Command for Enable {
    type Aggregate = account::Account;
    type Event = Enabled;
    type Context = PooledConnection<ConnectionManager<PgConnection>>;
    type Error = KernelError;

    fn validate(
        &self,
        _ctx: &Self::Context,
        aggregate: &Self::Aggregate,
    ) -> Result<(), Self::Error> {
        if aggregate.disabled_at.is_none() {
            return Err(KernelError::Validation(
                "Account is not disabled".to_string(),
            ));
        }

        if !self.actor.is_admin {
            return Err(KernelError::Forbidden("Admin role is required".to_string()));
        }

        return Ok(());
    }

    fn build_event(
        &self,
        _ctx: &Self::Context,
        _aggregate: &Self::Aggregate,
    ) -> Result<Self::Event, Self::Error> {
        return Ok(Enabled {
            timestamp: chrono::Utc::now(),
        });
    }
}

// Event
#[derive(Clone, Debug, Deserialize, EventTs, Serialize)]
pub struct Enabled {
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

impl Event for Enabled {
    type Aggregate = account::Account;

    fn apply(&self, aggregate: Self::Aggregate) -> Self::Aggregate {
        return Self::Aggregate {
            disabled_at: None,
            ..aggregate
        };
    }
}
