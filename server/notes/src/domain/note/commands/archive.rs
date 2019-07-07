use crate::domain::note;
use diesel::{
    r2d2::{ConnectionManager, PooledConnection},
    PgConnection,
};
use kernel::KernelError;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Archive {}

impl eventsourcing::Command for Archive {
    type Aggregate = note::Note;
    type Event = Archived;
    type Context = PooledConnection<ConnectionManager<PgConnection>>;
    type Error = KernelError;

    fn validate(
        &self,
        _ctx: &Self::Context,
        aggregate: &Self::Aggregate,
    ) -> Result<(), Self::Error> {
        if aggregate.deleted_at.is_some() {
            return Err(KernelError::NotFound("Note not found".to_string()));
        }

        if aggregate.archived_at.is_some() {
            return Err(KernelError::NotFound(
                "Not has already been archived".to_string(),
            ));
        }

        if aggregate.removed_at.is_some() {
            return Err(KernelError::NotFound(
                "Can't archive a note in trash".to_string(),
            ));
        }

        return Ok(());
    }

    fn build_event(
        &self,
        _ctx: &Self::Context,
        aggregate: &Self::Aggregate,
    ) -> Result<Self::Event, Self::Error> {
        return Ok(Archived {
            timestamp: chrono::Utc::now(),
            data: note::EventData::ArchivedV1,
            aggregate_id: aggregate.id,
            metadata: self.metadata.clone(),
        });
    }
}

// Event
#[derive(Clone, Debug, Deserialize, EventTs, Serialize)]
pub struct Archived {
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

impl Event for Archived {
    type Aggregate = note::Note;

    fn apply(&self, aggregate: Self::Aggregate) -> Self::Aggregate {
        return Self::Aggregate {
            archived_at: Some(self.timestamp),
            ..Aggregate
        };
    }
}
