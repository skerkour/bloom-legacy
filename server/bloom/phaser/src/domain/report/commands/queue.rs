use crate::{domain::report, domain::scan};
use diesel::{
    r2d2::{ConnectionManager, PooledConnection},
    PgConnection,
};
use eventsourcing::{Event, EventTs};
use kernel::KernelError;

#[derive(Clone, Debug)]
pub struct Queue {
    pub scan_id: uuid::Uuid,
    pub targets: Vec<String>,
    pub profile: scan::ScanProfile,
    pub trigger: scan::ReportTrigger,
}

impl eventsourcing::Command for Queue {
    type Aggregate = report::Report;
    type Event = Queued;
    type Context = PooledConnection<ConnectionManager<PgConnection>>;
    type Error = KernelError;

    fn validate(
        &self,
        _ctx: &Self::Context,
        _aggregate: &Self::Aggregate,
    ) -> Result<(), Self::Error> {
        return Ok(());
    }

    fn build_event(
        &self,
        _ctx: &Self::Context,
        _aggregate: &Self::Aggregate,
    ) -> Result<Self::Event, Self::Error> {
        return Ok(Queued {
            id: uuid::Uuid::new_v4(),
            timestamp: chrono::Utc::now(),
            scan_id: self.scan_id,
            targets: self.targets.clone(),
            profile: self.profile.clone(),
            trigger: self.trigger.clone(),
        });
    }
}

// Event
#[derive(Clone, Debug, EventTs)]
pub struct Queued {
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub id: uuid::Uuid,
    pub scan_id: uuid::Uuid,
    pub targets: Vec<String>,
    pub profile: scan::ScanProfile,
    pub trigger: scan::ReportTrigger,
}

impl Event for Queued {
    type Aggregate = report::Report;

    fn apply(&self, _aggregate: Self::Aggregate) -> Self::Aggregate {
        return Self::Aggregate {
            id: self.id,
            created_at: self.timestamp,
            updated_at: self.timestamp,
            version: 0,
            completed_at: None,
            error: None,
            findings: None,
            high_level_findings: 0,
            information_findings: 0,
            low_level_findings: 0,
            medium_level_findings: 0,
            profile: self.profile.clone(),
            started_at: None,
            status: report::ReportStatus::Queued,
            targets: self.targets.clone(),
            trigger: self.trigger.clone(),
            total_findings: 0,
            scan_id: self.scan_id,
        };
    }
}
