use crate::{
    accounts::domain::account,
    events::EventMetadata,
    accounts::validators,
    error::KernelError,
};
use diesel::{
    PgConnection,
    r2d2::{PooledConnection, ConnectionManager},
};


#[derive(Clone, Debug)]
pub struct UpdateEmail {
    pub email: String,
    pub metadata: EventMetadata,
}

impl eventsourcing::Command for UpdateEmail {
    type Aggregate = account::Account;
    type Event = account::Event;
    type Context = PooledConnection<ConnectionManager<PgConnection>>;
    type Error = KernelError;
    type NonStoredData = ();

    fn validate(&self, _ctx: &Self::Context, _aggregate: &Self::Aggregate) -> Result<(), Self::Error> {

        validators::email(&self.email)?;

        // verify that an email isn't already in use
        // already done in pending emial verify

        return Ok(());
    }

    fn build_event(&self, _ctx: &Self::Context, aggregate: &Self::Aggregate) -> Result<(Self::Event, Self::NonStoredData), Self::Error> {
        let data = account::EventData::EmailUpdatedV1(account::EmailUpdatedV1{
            email: self.email.clone(),
        });

        return  Ok((account::Event{
            id: uuid::Uuid::new_v4(),
            timestamp: chrono::Utc::now(),
            data,
            aggregate_id: aggregate.id,
            metadata: self.metadata.clone(),
        }, ()));
    }
}
