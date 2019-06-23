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
    domain::event,
    validators,
};


#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UpdateStartAt {
    pub start_at: chrono::DateTime<chrono::Utc>,
    pub metadata: EventMetadata,
}

impl eventsourcing::Command for UpdateStartAt {
    type Aggregate = event::CalendarEvent;
    type Event = event::Event;
    type Context = PooledConnection<ConnectionManager<PgConnection>>;
    type Error = KernelError;
    type NonStoredData = ();

    fn validate(&self, _ctx: &Self::Context, aggregate: &Self::Aggregate) -> Result<(), Self::Error> {
        if aggregate.deleted_at.is_some() {
            return Err(KernelError::NotFound("Event not found".to_string()));
        }

        validators::event_dates(self.start_at, aggregate.end_at)?;

        Ok(())
    }

    fn build_event(&self, _ctx: &Self::Context, aggregate: &Self::Aggregate) -> Result<(Self::Event, Self::NonStoredData), Self::Error> {
        let event_data = event::EventData::StartAtUpdatedV1(event::StartAtUpdatedV1{
            start_at: self.start_at,
        });

        Ok((event::Event{
            id: uuid::Uuid::new_v4(),
            timestamp: chrono::Utc::now(),
            data: event_data,
            aggregate_id: aggregate.id,
            metadata: self.metadata.clone(),
        }, ()))
    }
}
