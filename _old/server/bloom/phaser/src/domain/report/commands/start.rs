use crate::domain::report;
use diesel::{
    r2d2::{ConnectionManager, PooledConnection},
    PgConnection,
};
use eventsourcing::{Event, EventTs};
use kernel::KernelError;

#[derive(Clone, Debug)]
pub struct Start {}

impl eventsourcing::Command for Start {
    type Aggregate = report::Report;
    type Event = Started;
    type Context = PooledConnection<ConnectionManager<PgConnection>>;
    type Error = KernelError;

    fn validate(
        &self,
        _ctx: &Self::Context,
        aggregate: &Self::Aggregate,
    ) -> Result<(), Self::Error> {
        if aggregate.status != report::ReportStatus::Queued {
            return Err(KernelError::Validation("Report is not queued".to_string()));
        }

        return Ok(());
    }

    fn build_event(
        &self,
        _ctx: &Self::Context,
        _aggregate: &Self::Aggregate,
    ) -> Result<Self::Event, Self::Error> {
        return Ok(Started {
            timestamp: chrono::Utc::now(),
        });
    }
}

// Event
#[derive(Clone, Debug, EventTs)]
pub struct Started {
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

impl Event for Started {
    type Aggregate = report::Report;

    fn apply(&self, aggregate: Self::Aggregate) -> Self::Aggregate {
        return Self::Aggregate {
            started_at: Some(self.timestamp),
            status: report::ReportStatus::Scanning,
            ..aggregate
        };
    }
}
