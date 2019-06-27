use crate::{error::KernelError, events::EventMetadata, myaccount::domain::account};
use diesel::{
    r2d2::{ConnectionManager, PooledConnection},
    PgConnection,
};

#[derive(Clone, Debug)]
pub struct Disable {
    pub actor: account::Account,
    pub metadata: EventMetadata,
}

impl eventsourcing::Command for Disable {
    type Aggregate = account::Account;
    type Event = account::Event;
    type Context = PooledConnection<ConnectionManager<PgConnection>>;
    type Error = KernelError;
    type NonStoredData = ();

    fn validate(
        &self,
        _ctx: &Self::Context,
        aggregate: &Self::Aggregate,
    ) -> Result<(), Self::Error> {
        if aggregate.is_disabled {
            return Err(KernelError::Validation(
                "Account is already disabled".to_string(),
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
        aggregate: &Self::Aggregate,
    ) -> Result<(Self::Event, Self::NonStoredData), Self::Error> {
        return Ok((
            account::Event {
                id: uuid::Uuid::new_v4(),
                timestamp: chrono::Utc::now(),
                data: account::EventData::DisabledV1,
                aggregate_id: aggregate.id,
                metadata: self.metadata.clone(),
            },
            (),
        ));
    }
}
