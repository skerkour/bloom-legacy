use crate::{domain::note, validators};
use diesel::{
    r2d2::{ConnectionManager, PooledConnection},
    PgConnection,
};
use kernel::KernelError;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UpdateBody {
    pub body: String,
}

impl eventsourcing::Command for UpdateBody {
    type Aggregate = note::Note;
    type Event = BodyUpdated;
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
        validators::body(&self.body)?;

        return Ok(());
    }

    fn build_event(
        &self,
        _ctx: &Self::Context,
        aggregate: &Self::Aggregate,
    ) -> Result<Self::Event, Self::Error> {
        return Ok(BodyUpdated {
            timestamp: chrono::Utc::now(),
            body: self.body.clone(),
        });
    }
}

// Event
#[derive(Clone, Debug, Deserialize, EventTs, Serialize)]
pub struct BodyUpdated {
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub body: String,
}

impl Event for BodyUpdated {
    type Aggregate = note::Note;

    fn apply(&self, aggregate: Self::Aggregate) -> Self::Aggregate {
        return Self::Aggregate {
            body: self.body.clone(),
            ..Aggregate
        };
    }
}
