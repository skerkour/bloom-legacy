use serde::{Serialize, Deserialize};
use diesel::{
    PgConnection,
    r2d2::{PooledConnection, ConnectionManager},
};
use kernel::{
    KernelError,
    events::EventMetadata,
};
use crate::{
    domain::download,
};


#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Remove {
    pub metadata: EventMetadata,
}

impl eventsourcing::Command for Remove {
    type Aggregate = download::Download;
    type Event = download::Event;
    type Context = PooledConnection<ConnectionManager<PgConnection>>;
    type Error = KernelError;
    type NonStoredData = ();

    fn validate(&self, _ctx: &Self::Context, aggregate: &Self::Aggregate) -> Result<(), Self::Error> {
        if aggregate.deleted_at.is_some() {
            return Err(KernelError::NotFound("Download not found".to_string()));
        }

        if aggregate.removed_at.is_some() {
            return Err(KernelError::NotFound("Download has already been removed".to_string()));
        }

        return Ok(());
    }

    fn build_event(&self, _ctx: &Self::Context, aggregate: &Self::Aggregate) -> Result<(Self::Event, Self::NonStoredData), Self::Error> {
        return  Ok((download::Event{
            id: uuid::Uuid::new_v4(),
            timestamp: chrono::Utc::now(),
            data: download::EventData::RemovedV1,
            aggregate_id: aggregate.id,
            metadata: self.metadata.clone(),
        }, ()));
    }
}
