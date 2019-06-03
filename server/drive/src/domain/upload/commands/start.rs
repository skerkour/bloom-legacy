use diesel::{
    PgConnection,
    r2d2::{PooledConnection, ConnectionManager},
};
use kernel::{
    events::EventMetadata,
    KernelError,
};
use futures::future::Future;
use std::str::FromStr;
use crate::{
    domain::upload,
    validators,
};


#[derive(Clone, Debug)]
pub struct Start {
    pub file_name: String,
    pub parent_id: Option<uuid::Uuid>,
    pub owner_id: uuid::Uuid,
    pub metadata: EventMetadata,
}

impl eventsourcing::Command for Start {
    type Aggregate = upload::Upload;
    type Event = upload::Event;
    type Context = PooledConnection<ConnectionManager<PgConnection>>;
    type Error = KernelError;
    type NonStoredData = ();

    fn validate(&self, _ctx: &Self::Context, _aggregate: &Self::Aggregate) -> Result<(), Self::Error> {
        validators::file_name(&self.file_name)?;
        // TODO: check that parent exists

        return Ok(());
    }


    fn build_event(&self, _ctx: &Self::Context, _aggregate: &Self::Aggregate) -> Result<(Self::Event, Self::NonStoredData), Self::Error> {
        // presign request
        let id = uuid::Uuid::new_v4();
        let file_id = uuid::Uuid::new_v4();
        let key = format!("drive/{}/{}", self.owner_id, file_id);

        let event_data = upload::EventData::StartedV1(upload::StartedV1{
            id,
            file_name: self.file_name.clone(),
            file_id,
            parent_id: self.parent_id,
            owner_id: self.owner_id,
        });

        return  Ok((upload::Event{
            id: uuid::Uuid::new_v4(),
            timestamp: chrono::Utc::now(),
            data: event_data,
            aggregate_id: id,
            metadata: self.metadata.clone(),
        }, ()));
    }
}
