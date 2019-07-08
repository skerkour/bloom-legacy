use crate::domain::scan;
use diesel::{
    r2d2::{ConnectionManager, PooledConnection},
    PgConnection,
};
use eventsourcing::{Event, EventTs};
use kernel::KernelError;

#[derive(Clone, Debug)]
pub struct Complete {
    pub findings: u64,
}

impl eventsourcing::Command for Complete {
    type Aggregate = scan::Scan;
    type Event = Completed;
    type Context = PooledConnection<ConnectionManager<PgConnection>>;
    type Error = KernelError;

    fn validate(
        &self,
        _ctx: &Self::Context,
        aggregate: &Self::Aggregate,
    ) -> Result<(), Self::Error> {
        if aggregate.state != scan::ScanState::Scanning {
            return Err(KernelError::Validation("Scan is not running".to_string()));
        }

        return Ok(());
    }

    fn build_event(
        &self,
        _ctx: &Self::Context,
        _aggregate: &Self::Aggregate,
    ) -> Result<Self::Event, Self::Error> {
        return Ok(Completed {
            timestamp: chrono::Utc::now(),
            findings: self.findings,
        });
    }
}

// Event
#[derive(Clone, Debug, EventTs)]
pub struct Completed {
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub findings: u64,
}

impl Event for Completed {
    type Aggregate = scan::Scan;

    fn apply(&self, aggregate: Self::Aggregate) -> Self::Aggregate {
        return Self::Aggregate {
            state: scan::ScanState::Waiting,
            last: Some(self.timestamp),
            findings: self.findings as i64,
            ..aggregate
        };
    }
}
