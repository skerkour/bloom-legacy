use crate::domain::note;
use diesel::{
    r2d2::{ConnectionManager, PooledConnection},
    PgConnection,
};
use eventsourcing::{Event, EventTs};
use kernel::KernelError;

#[derive(Clone, Debug)]
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
        _aggregate: &Self::Aggregate,
    ) -> Result<(), Self::Error> {
        note::validators::body(&self.body)?;

        return Ok(());
    }

    fn build_event(
        &self,
        _ctx: &Self::Context,
        _aggregate: &Self::Aggregate,
    ) -> Result<Self::Event, Self::Error> {
        return Ok(BodyUpdated {
            timestamp: chrono::Utc::now(),
            body: self.body.clone(),
        });
    }
}

// Event
#[derive(Clone, Debug, EventTs)]
pub struct BodyUpdated {
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub body: String,
}

impl Event for BodyUpdated {
    type Aggregate = note::Note;

    fn apply(&self, aggregate: Self::Aggregate) -> Self::Aggregate {
        return Self::Aggregate {
            body: self.body.clone(),
            ..aggregate
        };
    }
}
