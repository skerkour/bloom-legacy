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
use super::ReportStatus;


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
    CreatedV1(CreatedV1),
    StartedV1,
}


#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreatedV1 {
    pub id: uuid::Uuid,
    pub scan_id: uuid::Uuid,
    pub targets: Vec<String>,
    pub profile:: ScanProfile,
    pub trigger: ReportTrigger,
}


impl eventsourcing::Event for Event {
    type Aggregate = super::Scan;

    fn apply(&self, aggregate: Self::Aggregate) -> Self::Aggregate {
        match self.data {
            // CreatedV1
            EventData::CreatedV1(ref data) => self::Aggregate{
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
            EventData::StartedV1 => self::Aggregate{
                started_at: self.timestamp,
                status: ReportStatus::Scanning,
                ..aggregate
            },
        }
    }

    fn timestamp(&self) -> chrono::DateTime<chrono::Utc> {
        return self.timestamp;
    }
}
