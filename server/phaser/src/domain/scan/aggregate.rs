use diesel::Queryable;
use kernel::db::schema::phaser_scans;
use serde::{Deserialize, Serialize};
use eventsourcing::Aggregate;

#[derive(Aggregate, AsChangeset, Clone, Debug, Deserialize, Identifiable, Insertable, Queryable, Serialize)]
#[table_name = "phaser_scans"]
#[changeset_options(treat_none_as_null = "true")]
pub struct Scan {
    pub id: uuid::Uuid,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub deleted_at: Option<chrono::DateTime<chrono::Utc>>,
    pub version: i64,

    pub description: String,
    pub findings: i64,
    pub last: Option<chrono::DateTime<chrono::Utc>>,
    pub name: String,
    pub profile: ScanProfile,
    pub schedule: ScanSchedule,
    pub state: ScanState,
    pub targets: Vec<String>,

    pub owner_id: uuid::Uuid,
}

#[derive(Clone, Debug, Deserialize, DieselEnum, PartialEq, Serialize)]
pub enum ScanProfile {
    Network,
    Application,
}

#[derive(Clone, Debug, Deserialize, DieselEnum, PartialEq, Serialize)]
pub enum ScanSchedule {
    Daily,
    Weekly,
    Monthly,
    Never,
}

#[derive(Clone, Debug, Deserialize, DieselEnum, PartialEq, Serialize)]
pub enum ScanState {
    Waiting,
    Queued,
    Scanning,
}

#[derive(Clone, Debug, Deserialize, DieselEnum, PartialEq, Serialize)]
pub enum ReportTrigger {
    Manual,
    Scheduled,
}

impl Scan {
    // create a new, unitialized note
    pub fn new() -> Self {
        let now = chrono::Utc::now();
        return Scan {
            id: uuid::Uuid::new_v4(),
            created_at: now,
            updated_at: now,
            deleted_at: None,
            version: 0,

            description: String::new(),
            findings: 0,
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

impl Default for Scan {
    fn default() -> Self {
        Self::new()
    }
}
