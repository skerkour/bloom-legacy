use crate::domain::scan;
use diesel::{
    r2d2::{ConnectionManager, PooledConnection},
    PgConnection,
};
use eventsourcing::{Event, EventTs};
use kernel::KernelError;

#[derive(Clone, Debug)]
pub struct Cancel {}

impl eventsourcing::Command for Cancel {
    type Aggregate = scan::Scan;
    type Event = Canceled;
    type Context = PooledConnection<ConnectionManager<PgConnection>>;
    type Error = KernelError;

    fn validate(
        &self,
        _ctx: &Self::Context,
        aggregate: &Self::Aggregate,
    ) -> Result<(), Self::Error> {
        if aggregate.state == scan::ScanState::Waiting {
            return Err(KernelError::Validation(
                "Scan is not running nor queued".to_string(),
            ));
        }

        return Ok(());
    }

    fn build_event(
        &self,
        _ctx: &Self::Context,
        _aggregate: &Self::Aggregate,
    ) -> Result<Self::Event, Self::Error> {
        return Ok(Canceled {
            timestamp: chrono::Utc::now(),
        });
    }
}

// Event
#[derive(Clone, Debug, EventTs)]
pub struct Canceled {
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

impl Event for Canceled {
    type Aggregate = scan::Scan;

    fn apply(&self, aggregate: Self::Aggregate) -> Self::Aggregate {
        return Self::Aggregate {
            state: scan::ScanState::Waiting,
            ..aggregate
        };
    }
}
