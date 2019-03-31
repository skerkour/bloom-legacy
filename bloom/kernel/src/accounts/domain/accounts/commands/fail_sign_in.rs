use diesel::{
    PgConnection,
    r2d2::{PooledConnection, ConnectionManager},
};
use crate::{
    accounts::domain::accounts,
    events::EventMetadata,
    error::KernelError,
};


#[derive(Clone, Debug)]
pub struct FailSignIn {
    pub metadata: EventMetadata,
}

impl<'a> eventsourcing::Command<'a> for FailSignIn {
    type Aggregate = accounts::Account;
    type Event = accounts::Event;
    type Context = PooledConnection<ConnectionManager<PgConnection>>;
    type Error = KernelError;
    type NonStoredData = ();

    fn validate(&self, _ctx: &Self::Context, _aggregate: &Self::Aggregate) -> Result<(), Self::Error> {
        return Ok(());
    }

    fn build_event(&self, _ctx: &Self::Context, aggregate: &Self::Aggregate) -> Result<(Self::Event, Self::NonStoredData), Self::Error> {
        return  Ok((accounts::Event{
            id: uuid::Uuid::new_v4(),
            timestamp: chrono::Utc::now(),
            data: accounts::EventData::SignInFailedV1,
            aggregate_id: aggregate.id,
            metadata: self.metadata.clone(),
        }, ()));
    }
}
