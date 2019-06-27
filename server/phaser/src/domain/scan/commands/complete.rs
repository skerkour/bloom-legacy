use crate::domain::scan;
use diesel::{
    r2d2::{ConnectionManager, PooledConnection},
    PgConnection,
};
use kernel::{events::EventMetadata, KernelError};

#[derive(Clone, Debug)]
pub struct Complete {
    pub findings: u64,
    pub metadata: EventMetadata,
}

impl eventsourcing::Command for Complete {
    type Aggregate = scan::Scan;
    type Event = scan::Event;
    type Context = PooledConnection<ConnectionManager<PgConnection>>;
    type Error = KernelError;
    type NonStoredData = ();

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
        aggregate: &Self::Aggregate,
    ) -> Result<(Self::Event, Self::NonStoredData), Self::Error> {
        let data = scan::EventData::CompletedV1(scan::CompletedV1 {
            findings: self.findings,
        });

        return Ok((
            scan::Event {
                id: uuid::Uuid::new_v4(),
                timestamp: chrono::Utc::now(),
                data,
                aggregate_id: aggregate.id,
                metadata: self.metadata.clone(),
            },
            (),
        ));
    }
}
