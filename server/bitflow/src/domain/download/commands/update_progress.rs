use crate::domain::download;
use diesel::{
    r2d2::{ConnectionManager, PooledConnection},
    PgConnection,
};
use eventsourcing::{Event, EventTs};
use kernel::KernelError;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UpdateProgress {
    pub progress: u32,
}

impl eventsourcing::Command for UpdateProgress {
    type Aggregate = download::Download;
    type Event = ProgressUpdated;
    type Context = PooledConnection<ConnectionManager<PgConnection>>;
    type Error = KernelError;

    fn validate(
        &self,
        _ctx: &Self::Context,
        aggregate: &Self::Aggregate,
    ) -> Result<(), Self::Error> {
        if aggregate.removed_at.is_some() {
            return Err(KernelError::Validation(
                "Download has been removed".to_string(),
            ));
        }

        if self.progress > 100 {
            return Err(KernelError::Validation(
                "Progress cannot be superior to 100".to_string(),
            ));
        }

        return Ok(());
    }

    fn build_event(
        &self,
        _ctx: &Self::Context,
        _aggregate: &Self::Aggregate,
    ) -> Result<Self::Event, Self::Error> {
        return Ok(ProgressUpdated {
            timestamp: chrono::Utc::now(),
            progress: self.progress,
        });
    }
}

// Event
#[derive(Clone, Debug, EventTs)]
pub struct ProgressUpdated {
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub progress: u32,
}

impl Event for ProgressUpdated {
    type Aggregate = download::Download;

    fn apply(&self, aggregate: Self::Aggregate) -> Self::Aggregate {
        return Self::Aggregate {
            progress: self.progress as i32,
            ..aggregate
        };
    }
}
