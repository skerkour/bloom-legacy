use crate::domain::download;
use diesel::{
    r2d2::{ConnectionManager, PooledConnection},
    PgConnection,
};
use eventsourcing::{Event, EventTs};
use kernel::KernelError;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Remove {}

impl eventsourcing::Command for Remove {
    type Aggregate = download::Download;
    type Event = Removed;
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
        return Ok(Removed {
            timestamp: chrono::Utc::now(),
        });
    }
}

// Event
#[derive(Clone, Debug, EventTs)]
pub struct Removed {
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

impl Event for Removed {
    type Aggregate = download::Download;

    fn apply(&self, aggregate: Self::Aggregate) -> Self::Aggregate {
        let status = if aggregate.status == download::DownloadStatus::Queued
            || aggregate.status == download::DownloadStatus::Downloading
        {
            download::DownloadStatus::Stopped
        } else {
            aggregate.status
        };
        return Self::Aggregate {
            removed_at: Some(self.timestamp),
            status,
            ..aggregate
        };
    }
}
