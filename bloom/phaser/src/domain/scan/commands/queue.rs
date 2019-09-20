use crate::domain::scan;
use diesel::{
    r2d2::{ConnectionManager, PooledConnection},
    PgConnection,
};
use eventsourcing::{Event, EventTs};
use kernel::KernelError;

#[derive(Clone, Debug)]
pub struct Queue {
    pub trigger: scan::ReportTrigger,
    pub report_id: uuid::Uuid,
}

impl eventsourcing::Command for Queue {
    type Aggregate = scan::Scan;
    type Event = Queued;
    type Context = PooledConnection<ConnectionManager<PgConnection>>;
    type Error = KernelError;

    fn validate(
        &self,
        _ctx: &Self::Context,
        aggregate: &Self::Aggregate,
    ) -> Result<(), Self::Error> {
        if aggregate.state != scan::ScanState::Waiting {
            return Err(KernelError::Validation(
                "Scan is already queued or running".to_string(),
            ));
        }

        return Ok(());
    }

    fn build_event(
        &self,
        _ctx: &Self::Context,
        _aggregate: &Self::Aggregate,
    ) -> Result<Self::Event, Self::Error> {
        return Ok(Queued {
            timestamp: chrono::Utc::now(),
            report_id: self.report_id,
            trigger: self.trigger.clone(),
        });
    }
}

// Event
#[derive(Clone, Debug, EventTs)]
pub struct Queued {
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub trigger: scan::ReportTrigger,
    pub report_id: uuid::Uuid,
}

impl Event for Queued {
    type Aggregate = scan::Scan;

    fn apply(&self, aggregate: Self::Aggregate) -> Self::Aggregate {
        return Self::Aggregate {
            state: scan::ScanState::Queued,
            ..aggregate
        };
    }
}
