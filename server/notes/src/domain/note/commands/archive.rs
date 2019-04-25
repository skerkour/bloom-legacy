use serde::{Serialize, Deserialize};
use diesel::{
    PgConnection,
    r2d2::{PooledConnection, ConnectionManager},
};
use kernel::{
    KernelError,
    events::EventMetadata,
};
use crate::domain::note;


#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Archive {
    pub metadata: EventMetadata,
}

impl eventsourcing::Command for Archive {
    type Aggregate = note::Note;
    type Event = note::Event;
    type Context = PooledConnection<ConnectionManager<PgConnection>>;
    type Error = KernelError;
    type NonStoredData = ();

    fn validate(&self, _ctx: &Self::Context, aggregate: &Self::Aggregate) -> Result<(), Self::Error> {
        if aggregate.deleted_at.is_some() {
            return Err(KernelError::NotFound("Note not found".to_string()));
        }

        if aggregate.archived_at.is_some() {
            return Err(KernelError::NotFound("Not has already been archived".to_string()));
        }

        if aggregate.removed_at.is_some() {
            return Err(KernelError::NotFound("Can't archive a note in trash".to_string()));
        }

        return Ok(());
    }

    fn build_event(&self, _ctx: &Self::Context, aggregate: &Self::Aggregate) -> Result<(Self::Event, Self::NonStoredData), Self::Error> {
        return  Ok((note::Event{
            id: uuid::Uuid::new_v4(),
            timestamp: chrono::Utc::now(),
            data: note::EventData::ArchivedV1,
            aggregate_id: aggregate.id,
            metadata: self.metadata.clone(),
        }, ()));
    }
}
