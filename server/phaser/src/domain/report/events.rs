use serde::{Deserialize, Serialize};
use diesel::{Queryable};
use diesel_as_jsonb::AsJsonb;
use kernel::{
    db::schema::phaser_reports_events,
    events::EventMetadata,
};
use crate::domain::scan::{
    ScanProfile,
    ReportTrigger,
};
use super::{
    ReportStatus,
    Finding,
};


#[derive(Clone, Debug, Deserialize, Insertable, Queryable, Serialize)]
#[table_name = "phaser_reports_events"]
pub struct Event {
    pub id: uuid::Uuid,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub data: EventData,
    pub aggregate_id: uuid::Uuid,
    pub metadata: EventMetadata,
}

#[derive(AsJsonb, Clone, Debug, Deserialize, Serialize)]
pub enum EventData {
    QueuedV1(QueuedV1),
    StartedV1,
    CompletedV1(CompletedV1),
    FailedV1(FailedV1),
    CanceledV1,
}


#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct QueuedV1 {
    pub id: uuid::Uuid,
    pub scan_id: uuid::Uuid,
    pub targets: Vec<String>,
    pub profile: ScanProfile,
    pub trigger: ReportTrigger,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CompletedV1 {
    pub findings: Vec<Finding>,
    pub high_level_findings: i64,
    pub information_findings: i64,
    pub low_level_findings: i64,
    pub medium_level_findings: i64,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct FailedV1 {
    pub error: String,
}


impl eventsourcing::Event for Event {
    type Aggregate = super::Report;

    fn apply(&self, aggregate: Self::Aggregate) -> Self::Aggregate {
        match self.data {
            // QueuedV1
            EventData::QueuedV1(ref data) => Self::Aggregate{
                id: data.id,
                created_at: self.timestamp,
                updated_at: self.timestamp,
                deleted_at: None,
                version: 0,

                completed_at: None,
                error: None,
                findings: None,
                high_level_findings: 0,
                information_findings: 0,
                low_level_findings: 0,
                medium_level_findings: 0,
                profile: data.profile.clone(),
                started_at: None,
                status: ReportStatus::Queued,
                targets: data.targets.clone(),
                trigger: data.trigger.clone(),

                scan_id: data.scan_id,
            },
            // StartedV1
            EventData::StartedV1 => Self::Aggregate{
                started_at: Some(self.timestamp),
                status: ReportStatus::Scanning,
                ..aggregate
            },
            // CanceledV1
            EventData::CanceledV1 => Self::Aggregate{
                status: ReportStatus::Canceled,
                ..aggregate
            },
            // CompletedV1
            EventData::CompletedV1(ref data) => Self::Aggregate{
                findings: Some(data.findings.clone()),
                high_level_findings: data.high_level_findings,
                information_findings: data.information_findings,
                low_level_findings: data.low_level_findings,
                medium_level_findings: data.medium_level_findings,
                ..aggregate
            },
            // FailedV1
            EventData::FailedV1(ref data) => Self::Aggregate{
                status: ReportStatus::Failed,
                error: Some(data.error.clone()),
                ..aggregate
            },
        }
    }

    fn timestamp(&self) -> chrono::DateTime<chrono::Utc> {
        return self.timestamp;
    }
}
