use crate::domain::scan;
use diesel::{
    r2d2::{ConnectionManager, PooledConnection},
    PgConnection,
};
use kernel::{events::EventMetadata, KernelError};

#[derive(Clone, Debug)]
pub struct Queue {
    pub trigger: scan::ReportTrigger,
    pub report_id: uuid::Uuid,
    pub metadata: EventMetadata,
}

impl eventsourcing::Command for Queue {
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
        aggregate: &Self::Aggregate,
    ) -> Result<(Self::Event, Self::NonStoredData), Self::Error> {
        let data = scan::EventData::QueuedV1(scan::QueuedV1 {
            report_id: self.report_id,
            trigger: self.trigger.clone(),
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
