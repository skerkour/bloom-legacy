use crate::domain::report;
use diesel::{
    r2d2::{ConnectionManager, PooledConnection},
    PgConnection,
};
use eventsourcing::{Event, EventTs};
use kernel::KernelError;

#[derive(Clone, Debug)]
pub struct Complete {
    pub findings: report::Finding,
    pub total_findings: u64,
}

impl eventsourcing::Command for Complete {
    type Aggregate = report::Report;
    type Event = Completed;
    type Context = PooledConnection<ConnectionManager<PgConnection>>;
    type Error = KernelError;

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
        _aggregate: &Self::Aggregate,
    ) -> Result<Self::Event, Self::Error> {
        return Ok(Completed {
            timestamp: chrono::Utc::now(),
            findings: vec![self.findings.clone()],
            high_level_findings: 0,
            information_findings: 0,
            low_level_findings: 0,
            medium_level_findings: 0,
            total_findings: self.total_findings,
        });
    }
}

// Event
#[derive(Clone, Debug, EventTs)]
pub struct Completed {
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub findings: Vec<report::Finding>,
    pub high_level_findings: u64,
    pub information_findings: u64,
    pub low_level_findings: u64,
    pub medium_level_findings: u64,
    pub total_findings: u64,
}

impl Event for Completed {
    type Aggregate = report::Report;

    fn apply(&self, aggregate: Self::Aggregate) -> Self::Aggregate {
        return Self::Aggregate {
            findings: Some(self.findings.clone()),
            high_level_findings: self.high_level_findings as i64,
            information_findings: self.information_findings as i64,
            low_level_findings: self.low_level_findings as i64,
            medium_level_findings: self.medium_level_findings as i64,
            total_findings: self.total_findings as i64,
            status: report::ReportStatus::Success,
            completed_at: Some(self.timestamp),
            ..aggregate
        };
    }
}
