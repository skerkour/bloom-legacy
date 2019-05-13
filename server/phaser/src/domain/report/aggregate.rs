use serde::{Serialize, Deserialize};
use diesel::{Queryable};
use kernel::{
    db::schema::phaser_reports,
};
use crate::domain::scan::{
    ScanProfile,
    ReportTrigger,
};


#[derive(AsChangeset, Clone, Debug, Deserialize, Identifiable, Insertable, Queryable, Serialize)]
#[table_name = "phaser_reports"]
#[changeset_options(treat_none_as_null = "true")]
pub struct Report {
    pub id: uuid::Uuid,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub deleted_at: Option<chrono::DateTime<chrono::Utc>>,
    pub version: i64,

    pub completed_at: Option<chrono::DateTime<chrono::Utc>>,
    pub findings: Vec<Finding>,
    pub high_level_findings: i64,
    pub information_findings: i64,
    pub low_level_findings: i64,
    pub medium_level_findings: i64,
    pub profile: ScanProfile,
    pub started_at: Option<chrono::DateTime<chrono::Utc>>,
    pub status: ReportStatus,
    pub targets: Vec<String>,
    pub trigger: ReportTrigger,

    pub scan_id: uuid::Uuid,
}

#[derive(Clone, Debug, Deserialize, DieselEnum, PartialEq, Serialize)]
pub enum ReportStatus {
    Scanning,
    Success,
    Failed,
    Stopped, // by user
}

#[derive(Clone, Debug, Deserialize, DieselEnum, PartialEq, Serialize)]
pub enum ScanState {
    Waiting,
    Queued,
    Scanning,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum Finding {
    A,
    B,
}

impl Scan {
    // create a new, unitialized note
    pub fn new() -> Self {
        let now = chrono::Utc::now();
        return Scan{
            id: uuid::Uuid::new_v4(),
            created_at: now,
            updated_at: now,
            deleted_at: None,
            version: 0,

            description: String::new(),
            last: None,
            name: String::new(),
            profile: ScanProfile::Application,
            schedule: ScanSchedule::Never,
            state: ScanState::Waiting,
            targets: Vec::new(),

            owner_id: uuid::Uuid::new_v4(),
        };
    }
}

impl eventsourcing::Aggregate for Scan {
    fn increment_version(&mut self) {
        self.version += 1;
    }

    fn update_updated_at(&mut self, timestamp: chrono::DateTime<chrono::Utc>) {
        self.updated_at = timestamp;
    }
}
