use crate::domain::report;
use diesel::{
    r2d2::{ConnectionManager, PooledConnection},
    PgConnection,
};
use kernel::{events::EventMetadata, KernelError};

#[derive(Clone, Debug)]
pub struct Complete {
    pub findings: report::Finding,
    pub total_findings: u64,
    pub metadata: EventMetadata,
}

impl eventsourcing::Command for Complete {
    type Aggregate = report::Report;
    type Event = report::Event;
    type Context = PooledConnection<ConnectionManager<PgConnection>>;
    type Error = KernelError;
    type NonStoredData = ();

    fn validate(
        &self,
        _ctx: &Self::Context,
        aggregate: &Self::Aggregate,
    ) -> Result<(), Self::Error> {
        if aggregate.status != report::ReportStatus::Scanning {
            return Err(KernelError::Validation("Report is not running".to_string()));
        }

        return Ok(());
    }

    fn build_event(
        &self,
        _ctx: &Self::Context,
        aggregate: &Self::Aggregate,
    ) -> Result<(Self::Event, Self::NonStoredData), Self::Error> {
        let data = report::EventData::CompletedV1(report::CompletedV1 {
            findings: vec![self.findings.clone()],
            high_level_findings: 0,
            information_findings: 0,
            low_level_findings: 0,
            medium_level_findings: 0,
            total_findings: self.total_findings,
        });

        return Ok((
            report::Event {
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
