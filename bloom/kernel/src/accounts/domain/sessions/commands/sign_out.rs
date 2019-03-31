use serde::{Serialize, Deserialize};
use crate::{
    accounts::domain::sessions,
    error::KernelError,
    events::EventMetadata,
};
use diesel::{
    PgConnection,
    r2d2::{PooledConnection, ConnectionManager},
};


#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SignOut {
    pub metadata: EventMetadata,
}

impl<'a> eventsourcing::Command<'a> for SignOut {
    type Aggregate = sessions::Session;
    type Event = sessions::Event;
    type Context = PooledConnection<ConnectionManager<PgConnection>>;
    type Error = KernelError;
    type NonStoredData = ();

    fn validate(&self, _ctx: &Self::Context, aggregate: &Self::Aggregate) -> Result<(), Self::Error> {
        if aggregate.deleted_at.is_some() {
            return Err(KernelError::Validation("Session is currently not active.".to_string()))
        }
        return Ok(());
    }

    fn build_event(&self, _ctx: &Self::Context, aggregate: &Self::Aggregate) -> Result<(Self::Event, Self::NonStoredData), Self::Error> {
        let data = sessions::EventData::SignedOutV1;
        let timestamp = chrono::Utc::now();

        return  Ok((sessions::Event{
            id: uuid::Uuid::new_v4(),
            timestamp,
            data,
            aggregate_id: aggregate.id,
            metadata: self.metadata.clone(),
        }, ()));
    }
}
