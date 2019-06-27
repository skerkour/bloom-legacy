use crate::domain::file;
use diesel::{
    r2d2::{ConnectionManager, PooledConnection},
    PgConnection,
};
use kernel::{events::EventMetadata, KernelError};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Upload {
    pub id: uuid::Uuid,
    pub name: String,
    pub parent_id: Option<uuid::Uuid>,
    pub size: i64,
    #[serde(rename = "type")]
    pub type_: String, // MIME type
    pub owner_id: uuid::Uuid,
    pub metadata: EventMetadata,
}

impl eventsourcing::Command for Upload {
    type Aggregate = file::File;
    type Event = file::Event;
    type Context = PooledConnection<ConnectionManager<PgConnection>>;
    type Error = KernelError;
    type NonStoredData = ();

    fn validate(
        &self,
        _ctx: &Self::Context,
        _aggregate: &Self::Aggregate,
    ) -> Result<(), Self::Error> {
        // TODO
        return Ok(());
    }

    fn build_event(
        &self,
        _ctx: &Self::Context,
        _aggregate: &Self::Aggregate,
    ) -> Result<(Self::Event, Self::NonStoredData), Self::Error> {
        let event_data = file::EventData::UploadedV1(file::UploadedV1 {
            id: self.id,
            parent_id: self.parent_id,
            name: self.name.clone(),
            size: self.size,
            type_: self.type_.clone(), // MIME type
            owner_id: self.owner_id,
        });

        return Ok((
            file::Event {
                id: uuid::Uuid::new_v4(),
                timestamp: chrono::Utc::now(),
                data: event_data,
                aggregate_id: self.id,
                metadata: self.metadata.clone(),
            },
            (),
        ));
    }
}
