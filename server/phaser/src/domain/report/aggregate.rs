use crate::{
    domain::scan::{ReportTrigger, ScanProfile},
    models,
};
use diesel::Queryable;
use diesel_as_jsonb::AsJsonb;
use eventsourcing::Aggregate;
use kernel::db::schema::phaser_reports;
use serde::{Deserialize, Serialize};

#[derive(
    Aggregate,
    AsChangeset,
    Clone,
    Debug,
    Deserialize,
    Identifiable,
    Insertable,
    Queryable,
    Serialize,
)]
#[table_name = "phaser_reports"]
#[changeset_options(treat_none_as_null = "true")]
pub struct Report {
    pub id: uuid::Uuid,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub version: i64,

    pub completed_at: Option<chrono::DateTime<chrono::Utc>>,
    pub error: Option<String>,
    pub findings: Option<Vec<Finding>>,
    pub high_level_findings: i64,
    pub information_findings: i64,
    pub low_level_findings: i64,
    pub medium_level_findings: i64,
    pub profile: ScanProfile,
    pub started_at: Option<chrono::DateTime<chrono::Utc>>,
    pub status: ReportStatus,
    pub targets: Vec<String>,
    pub total_findings: i64,
    pub trigger: ReportTrigger,

    pub scan_id: uuid::Uuid,
}

#[derive(Clone, Debug, Deserialize, DieselEnum, PartialEq, Serialize)]
pub enum ReportStatus {
    Queued,
    Scanning,
    Success,
    Failed,
    Canceled, // by user
}

#[derive(AsJsonb, Clone, Debug, Deserialize, Serialize)]
pub enum Finding {
    V1(models::report::ReportV1),
}

impl Report {
    // create a new, unitialized note
    pub fn new() -> Self {
        let now = chrono::Utc::now();
        return Self {
            id: uuid::Uuid::new_v4(),
            created_at: now,
            updated_at: now,
            version: 0,

            completed_at: None,
            error: None,
            findings: None,
            high_level_findings: 0,
            information_findings: 0,
            low_level_findings: 0,
            medium_level_findings: 0,
            profile: ScanProfile::Application,
            started_at: None,
            status: ReportStatus::Canceled,
            targets: Vec::new(),
            trigger: ReportTrigger::Manual,
            total_findings: 0,

            scan_id: uuid::Uuid::new_v4(),
        };
    }
}

impl Default for Report {
    fn default() -> Self {
        Self::new()
    }
}

/*

            // FailedV1
            EventData::FailedV1(ref data) => Self::Aggregate {
                status: ReportStatus::Failed,
                error: Some(data.error.clone()),
                ..aggregate
            },
*/
