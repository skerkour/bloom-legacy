use serde::{Serialize, Deserialize};
use diesel::{
    PgConnection,
    r2d2::{PooledConnection, ConnectionManager},
};
use kernel::{
    events::EventMetadata,
    KernelError,
};
use crate::{
    domain::file,
    validators,
};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Upload {
    pub name: String,
    pub parent_id: Option<uuid::Uuid>,
    pub size: i64,
     #[serde(rename = "type")]
    pub type_: String, // MIME type
    pub owner_id: uuid::Uuid,
    pub metadata: EventMetadata,
}

impl<'a> eventsourcing::Command<'a> for Upload {
    type Aggregate = file::File;
    type Event = file::Event;
    type Context = PooledConnection<ConnectionManager<PgConnection>>;
    type Error = KernelError;
    type NonStoredData = ();

    fn validate(&self, _ctx: &Self::Context, _aggregate: &Self::Aggregate) -> Result<(), Self::Error> {
        // TODO
        return Ok(());
    }

    fn build_event(&self, _ctx: &Self::Context, aggregate: &Self::Aggregate) -> Result<(Self::Event, Self::NonStoredData), Self::Error> {
        let id = uuid::Uuid::new_v4();

        let event_data = file::EventData::UploadedV1(file::UploadedV1{
            id,
            parent_id: self.parent_id,
            name: self.name.clone(),
            size: self.size,
            type_: self.type_.clone(), // MIME type
            owner_id: self.owner_id,
        });

        return  Ok((file::Event{
            id: uuid::Uuid::new_v4(),
            timestamp: chrono::Utc::now(),
            data: event_data,
            aggregate_id: aggregate.id,
            metadata: self.metadata.clone(),
        }, ()));
    }
}
