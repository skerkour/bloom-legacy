use crate::{domain::event, validators};
use diesel::{
    r2d2::{ConnectionManager, PooledConnection},
    PgConnection,
};
use kernel::{events::EventMetadata, KernelError};

#[derive(Clone, Debug)]
pub struct Create {
    pub title: String,
    pub description: String,
    pub start_at: chrono::DateTime<chrono::Utc>,
    pub end_at: chrono::DateTime<chrono::Utc>,
    pub owner_id: uuid::Uuid,
    pub metadata: EventMetadata,
}

impl eventsourcing::Command for Create {
    type Aggregate = event::CalendarEvent;
    type Event = event::Event;
    type Context = PooledConnection<ConnectionManager<PgConnection>>;
    type Error = KernelError;
    type NonStoredData = ();

    // Validate to implement the goes.Command interface
    fn validate(
        &self,
        _ctx: &Self::Context,
        _aggregate: &Self::Aggregate,
    ) -> Result<(), Self::Error> {
        validators::event_dates(self.start_at, self.end_at)?;
        validators::event_title(&self.title)?;
        validators::event_description(&self.description)?;
        Ok(())
    }

    fn build_event(
        &self,
        _ctx: &Self::Context,
        _aggregate: &Self::Aggregate,
    ) -> Result<(Self::Event, Self::NonStoredData), Self::Error> {
        let id = uuid::Uuid::new_v4();
        let event_data = event::EventData::CreatedV1(event::CreatedV1 {
            id,
            title: self.title.clone(),
            description: self.description.clone(),
            start_at: self.start_at,
            end_at: self.end_at,
            owner_id: self.owner_id,
        });

        return Ok((
            event::Event {
                id: uuid::Uuid::new_v4(),
                timestamp: chrono::Utc::now(),
                data: event_data,
                aggregate_id: id,
                metadata: self.metadata.clone(),
            },
            (),
        ));
    }
}
