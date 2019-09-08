use crate::domain::download;
use diesel::{
    r2d2::{ConnectionManager, PooledConnection},
    PgConnection,
};
use eventsourcing::{Event, EventTs};
use kernel::KernelError;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Fail {
    pub error: String,
}

impl eventsourcing::Command for Fail {
    type Aggregate = download::Download;
    type Event = Failed;
    type Context = PooledConnection<ConnectionManager<PgConnection>>;
    type Error = KernelError;

    fn validate(
        &self,
        _ctx: &Self::Context,
        aggregate: &Self::Aggregate,
    ) -> Result<(), Self::Error> {
        if aggregate.removed_at.is_some() {
            return Err(KernelError::NotFound(
                "Download has already been removed".to_string(),
            ));
        }

        return Ok(());
    }

    fn build_event(
        &self,
        _ctx: &Self::Context,
        _aggregate: &Self::Aggregate,
    ) -> Result<Self::Event, Self::Error> {
        return Ok(Failed {
            timestamp: chrono::Utc::now(),
            error: self.error.clone(),
        });
    }
}

// Event
#[derive(Clone, Debug, EventTs)]
pub struct Failed {
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub error: String,
}

impl Event for Failed {
    type Aggregate = download::Download;

    fn apply(&self, aggregate: Self::Aggregate) -> Self::Aggregate {
        return Self::Aggregate {
            error: Some(self.error.clone()),
            status: download::DownloadStatus::Failed,
            ..aggregate
        };
    }
}
