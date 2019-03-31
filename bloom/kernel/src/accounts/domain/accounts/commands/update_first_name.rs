use crate::{
    accounts::domain::accounts,
    events::EventMetadata,
    accounts::validators,
    error::KernelError,
};
use diesel::{
    PgConnection,
    r2d2::{PooledConnection, ConnectionManager},
};


#[derive(Clone, Debug)]
pub struct UpdateFirstName {
    pub first_name: String,
    pub metadata: EventMetadata,
}

impl<'a> eventsourcing::Command<'a> for UpdateFirstName {
    type Aggregate = accounts::Account;
    type Event = accounts::Event;
    type Context = PooledConnection<ConnectionManager<PgConnection>>;
    type Error = KernelError;
    type NonStoredData = ();

    fn validate(&self, _ctx: &Self::Context, _aggregate: &Self::Aggregate) -> Result<(), Self::Error> {
        validators::first_name(&self.first_name)?;

        return Ok(());
    }

    fn build_event(&self, _ctx: &Self::Context, aggregate: &Self::Aggregate) -> Result<(Self::Event, Self::NonStoredData), Self::Error> {
        let data = accounts::EventData::FirstNameUpdatedV1(accounts::FirstNameUpdatedV1{
            first_name: self.first_name.clone(),
        });

        return  Ok((accounts::Event{
            id: uuid::Uuid::new_v4(),
            timestamp: chrono::Utc::now(),
            data,
            aggregate_id: aggregate.id,
            metadata: self.metadata.clone(),
        }, ()));
    }
}
