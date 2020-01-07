use crate::{error::KernelError, events::EventMetadata, accounts::domain::account};
use diesel::{
    r2d2::{ConnectionManager, PooledConnection},
    PgConnection,
};

#[derive(Clone, Debug)]
pub struct FailSignIn {
    pub metadata: EventMetadata,
}

impl eventsourcing::Command for FailSignIn {
    type Aggregate = account::Account;
    type Event = account::Event;
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
        aggregate: &Self::Aggregate,
    ) -> Result<Self::Event, Self::Error> {
        return Ok(account::Event {
            id: uuid::Uuid::new_v4(),
            timestamp: chrono::Utc::now(),
            data: account::EventData::SignInFailedV1,
            aggregate_id: aggregate.id,
            metadata: self.metadata.clone(),
        });
    }
}
