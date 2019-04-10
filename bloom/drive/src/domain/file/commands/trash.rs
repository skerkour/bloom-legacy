use diesel::{
    PgConnection,
    r2d2::{PooledConnection, ConnectionManager},
};
use kernel::{
    KernelError,
    events::EventMetadata,
};
use crate::{
    domain::file,
    FOLDER_TYPE,
};


#[derive(Clone, Debug)]
pub struct Trash {
    pub explicitly_trashed: bool,
    pub metadata: EventMetadata,
}

impl eventsourcing::Command for Trash {
    type Aggregate = file::File;
    type Event = file::Event;
    type Context = PooledConnection<ConnectionManager<PgConnection>>;
    type Error = KernelError;
    type NonStoredData = ();

    fn validate(&self, ctx: &Self::Context, aggregate: &Self::Aggregate) -> Result<(), Self::Error> {
        use kernel::db::schema::{
            drive_files,
            drive_files_events,
        };
        use diesel::prelude::*;

        if aggregate.trashed_at.is_some() {
            return Err(KernelError::Validation("File is already in trash".to_string()))
        }

        return Ok(());
    }

    fn build_event(&self, _ctx: &Self::Context, aggregate: &Self::Aggregate) -> Result<(Self::Event, Self::NonStoredData), Self::Error> {
        let event_data = file::EventData::TrashedV1(file::TrashedV1{
            explicitly_trashed: self.explicitly_trashed,
        });;

        return  Ok((file::Event{
            id: uuid::Uuid::new_v4(),
            timestamp: chrono::Utc::now(),
            data: event_data,
            aggregate_id: aggregate.id,
            metadata: self.metadata.clone(),
        }, ()));
    }
}
