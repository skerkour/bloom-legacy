use crate::domain::note;
use diesel::{
    r2d2::{ConnectionManager, PooledConnection},
    PgConnection,
};
use eventsourcing::{Event, EventTs};
use kernel::KernelError;

#[derive(Clone, Debug)]
pub struct Create {
    pub title: String,
    pub body: String,
    pub owner_id: uuid::Uuid,
}

impl eventsourcing::Command for Create {
    type Aggregate = note::Note;
    type Event = Created;
    type Context = PooledConnection<ConnectionManager<PgConnection>>;
    type Error = KernelError;

    fn validate(
        &self,
        _ctx: &Self::Context,
        _aggregate: &Self::Aggregate,
    ) -> Result<(), Self::Error> {
        note::validators::title(&self.title)?;
        note::validators::body(&self.body)?;

        return Ok(());
    }

    fn build_event(
        &self,
        _ctx: &Self::Context,
        _aggregate: &Self::Aggregate,
    ) -> Result<Self::Event, Self::Error> {
        return Ok(Created {
            id: uuid::Uuid::new_v4(),
            timestamp: chrono::Utc::now(),
            title: self.title.clone(),
            body: self.body.clone(),
            owner_id: self.owner_id,
        });
    }
}

// Event
#[derive(Clone, Debug, EventTs)]
pub struct Created {
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub id: uuid::Uuid,
    pub title: String,
    pub body: String,
    pub owner_id: uuid::Uuid,
}

impl Event for Created {
    type Aggregate = note::Note;

    fn apply(&self, _aggregate: Self::Aggregate) -> Self::Aggregate {
        return Self::Aggregate {
            id: self.id,
            created_at: self.timestamp,
            updated_at: self.timestamp,
            version: 0,
            archived_at: None,
            body: self.body.clone(),
            removed_at: None,
            title: self.title.clone(),
            owner_id: self.owner_id,
        };
    }
}
