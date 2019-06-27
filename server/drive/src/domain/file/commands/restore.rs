use crate::domain::file;
use diesel::{
    r2d2::{ConnectionManager, PooledConnection},
    PgConnection,
};
use kernel::{events::EventMetadata, KernelError};

#[derive(Clone, Debug)]
pub struct Restore {
    pub metadata: EventMetadata,
}

impl eventsourcing::Command for Restore {
    type Aggregate = file::File;
    type Event = file::Event;
    type Context = PooledConnection<ConnectionManager<PgConnection>>;
    type Error = KernelError;
    type NonStoredData = ();

    fn validate(
        &self,
        _ctx: &Self::Context,
        aggregate: &Self::Aggregate,
    ) -> Result<(), Self::Error> {
        if aggregate.trashed_at.is_none() {
            return Err(KernelError::Validation("File is not in trash".to_string()));
        }

        return Ok(());
    }

    fn build_event(
        &self,
        _ctx: &Self::Context,
        aggregate: &Self::Aggregate,
    ) -> Result<(Self::Event, Self::NonStoredData), Self::Error> {
        let event_data = file::EventData::RestoredV1;

        return Ok((
            file::Event {
                id: uuid::Uuid::new_v4(),
                timestamp: chrono::Utc::now(),
                data: event_data,
                aggregate_id: aggregate.id,
                metadata: self.metadata.clone(),
            },
            (),
        ));
    }
}
