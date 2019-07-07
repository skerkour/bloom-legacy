use crate::{domain::event, validators};
use diesel::{
    r2d2::{ConnectionManager, PooledConnection},
    PgConnection,
};
use kernel::KernelError;

#[derive(Clone, Debug)]
pub struct Create {
    pub title: String,
    pub description: String,
    pub start_at: chrono::DateTime<chrono::Utc>,
    pub end_at: chrono::DateTime<chrono::Utc>,
    pub owner_id: uuid::Uuid,
}

impl eventsourcing::Command for Create {
    type Aggregate = event::CalendarEvent;
    type Event = Created;
    type Context = PooledConnection<ConnectionManager<PgConnection>>;
    type Error = KernelError;

    // Validate to implement the goes.Command interface
    fn validate(
        &self,
        _ctx: &Self::Context,
        _aggregate: &Self::Aggregate,
    ) -> Result<(), Self::Error> {
        validators::event_dates(self.start_at, self.end_at)?;
        validators::event_title(&self.title)?;
        validators::event_description(&self.description)?;
        return Ok(());
    }

    fn build_event(
        &self,
        _ctx: &Self::Context,
        _aggregate: &Self::Aggregate,
    ) -> Result<Self::Event, Self::Error> {
        return Ok(Created {
            id: uuid::Uuid::new_v4(),
            title: self.title.clone(),
            description: self.description.clone(),
            start_at: self.start_at,
            end_at: self.end_at,
            owner_id: self.owner_id,
        });
    }
}

// Event
#[derive(Clone, Debug, Deserialize, EventTs, Serialize)]
pub struct CreatedV1 {
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub id: uuid::Uuid,
    pub title: String,
    pub description: String,
    pub start_at: chrono::DateTime<chrono::Utc>,
    pub end_at: chrono::DateTime<chrono::Utc>,
    pub owner_id: uuid::Uuid,
}

impl Event for Created {
    type Aggregate = account::Account;

    fn apply(&self, _aggregate: Self::Aggregate) -> Self::Aggregate {
        return Self::Aggregate {
            id: self.id,
            created_at: self.timestamp,
            updated_at: self.timestamp,
            deleted_at: None,
            version: 0,
            title: self.title.clone(),
            description: self.description.clone(),
            start_at: self.start_at,
            end_at: self.end_at,
            owner_id: self.owner_id,
        };
    }
}
