use crate::{
    users::domain::user,
    events::EventMetadata,
    users::validators,
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
    type Aggregate = user::User;
    type Event = user::Event;
    type Context = PooledConnection<ConnectionManager<PgConnection>>;
    type Error = KernelError;
    type NonStoredData = ();

    fn validate(&self, ctx: &Self::Context, _aggregate: &Self::Aggregate) -> Result<(), Self::Error> {
        validators::first_name(&self.first_name)?;

        return Ok(());
    }

    fn build_event(&self, _ctx: &Self::Context, aggregate: &Self::Aggregate) -> Result<(Self::Event, Self::NonStoredData), Self::Error> {
        let data = user::EventData::FirstNameUpdatedV1(user::FirstNameUpdatedV1{
            first_name: self.first_name.clone(),
        });

        return  Ok((user::Event{
            id: uuid::Uuid::new_v4(),
            timestamp: chrono::Utc::now(),
            data,
            aggregate_id: aggregate.id,
            metadata: self.metadata.clone(),
        }, ()));
    }
}
