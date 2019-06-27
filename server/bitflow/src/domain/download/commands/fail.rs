use crate::domain::download;
use diesel::{
    r2d2::{ConnectionManager, PooledConnection},
    PgConnection,
};
use kernel::{events::EventMetadata, KernelError};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Fail {
    pub error: String,
    pub metadata: EventMetadata,
}

impl eventsourcing::Command for Fail {
    type Aggregate = download::Download;
    type Event = download::Event;
    type Context = PooledConnection<ConnectionManager<PgConnection>>;
    type Error = KernelError;
    type NonStoredData = ();

    fn validate(
        &self,
        _ctx: &Self::Context,
        aggregate: &Self::Aggregate,
    ) -> Result<(), Self::Error> {
        if aggregate.deleted_at.is_some() {
            return Err(KernelError::NotFound("Download not found".to_string()));
        }

        if aggregate.removed_at.is_some() {
            return Err(KernelError::NotFound(
                "Download has already been removed".to_string(),
            ));
        }

        return Ok(());
    }

    fn build_event(
        &self,
        _ctx: &Self::Context,
        aggregate: &Self::Aggregate,
    ) -> Result<(Self::Event, Self::NonStoredData), Self::Error> {
        let event_data = download::EventData::FailedV1(download::FailedV1 {
            error: self.error.clone(),
        });

        return Ok((
            download::Event {
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
