use crate::{domain::event, validators};
use diesel::{
    r2d2::{ConnectionManager, PooledConnection},
    PgConnection,
};
use kernel::{events::EventMetadata, KernelError};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UpdateEndAt {
    pub end_at: chrono::DateTime<chrono::Utc>,
    pub metadata: EventMetadata,
}

impl eventsourcing::Command for UpdateEndAt {
    type Aggregate = event::CalendarEvent;
    type Event = event::Event;
    type Context = PooledConnection<ConnectionManager<PgConnection>>;
    type Error = KernelError;
    type NonStoredData = ();

    fn validate(
        &self,
        _ctx: &Self::Context,
        aggregate: &Self::Aggregate,
    ) -> Result<(), Self::Error> {
        if aggregate.deleted_at.is_some() {
            return Err(KernelError::NotFound("Event not found".to_string()));
        }

        validators::event_dates(aggregate.start_at, self.end_at)?;

        Ok(())
    }

    fn build_event(
        &self,
        _ctx: &Self::Context,
        aggregate: &Self::Aggregate,
    ) -> Result<(Self::Event, Self::NonStoredData), Self::Error> {
        let event_data = event::EventData::EndAtUpdatedV1(event::EndAtUpdatedV1 {
            end_at: self.end_at,
        });

        Ok((
            event::Event {
                id: uuid::Uuid::new_v4(),
                timestamp: chrono::Utc::now(),
                data: event_data,
                aggregate_id: aggregate.id,
                metadata: self.metadata.clone(),
            },
            (),
        ))
    }
}
