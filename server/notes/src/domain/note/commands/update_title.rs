use crate::{domain::note, validators};
use diesel::{
    r2d2::{ConnectionManager, PooledConnection},
    PgConnection,
};
use kernel::{events::EventMetadata, KernelError};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UpdateTitle {
    pub title: String,
    pub metadata: EventMetadata,
}

impl eventsourcing::Command for UpdateTitle {
    type Aggregate = note::Note;
    type Event = note::Event;
    type Context = PooledConnection<ConnectionManager<PgConnection>>;
    type Error = KernelError;
    type NonStoredData = ();

    fn validate(
        &self,
        _ctx: &Self::Context,
        aggregate: &Self::Aggregate,
    ) -> Result<(), Self::Error> {
        if aggregate.deleted_at.is_some() {
            return Err(KernelError::NotFound("Note not found".to_string()));
        }

        validators::title(&self.title)?;

        return Ok(());
    }

    fn build_event(
        &self,
        _ctx: &Self::Context,
        aggregate: &Self::Aggregate,
    ) -> Result<(Self::Event, Self::NonStoredData), Self::Error> {
        let data = note::EventData::TitleUpdatedV1(note::TitleUpdatedV1 {
            title: self.title.clone(),
        });

        return Ok((
            note::Event {
                id: uuid::Uuid::new_v4(),
                timestamp: chrono::Utc::now(),
                data,
                aggregate_id: aggregate.id,
                metadata: self.metadata.clone(),
            },
            (),
        ));
    }
}
