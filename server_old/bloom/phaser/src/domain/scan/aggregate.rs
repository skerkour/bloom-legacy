use diesel::Queryable;
use eventsourcing::Aggregate;
use kernel::db::schema::phaser_scans;
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
#[table_name = "phaser_scans"]
#[changeset_options(treat_none_as_null = "true")]
pub struct Scan {
    pub id: uuid::Uuid,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
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

/*
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct NameUpdatedV1 {
    pub name: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DescriptionUpdatedV1 {
    pub description: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ScheduleUpdatedV1 {
    pub schedule: ScanSchedule,
}

 // NameUpdatedV1
            EventData::NameUpdatedV1(ref data) => super::Scan {
                name: data.name.clone(),
                ..aggregate
            },
            // DescriptionUpdatedV1
            EventData::DescriptionUpdatedV1(ref data) => super::Scan {
                description: data.description.clone(),
                ..aggregate
            },
            // ScheduleUpdatedV1
            EventData::ScheduleUpdatedV1(ref data) => super::Scan {
                schedule: data.schedule.clone(),
                ..aggregate

*/
