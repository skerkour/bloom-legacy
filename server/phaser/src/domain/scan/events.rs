use serde::{Deserialize, Serialize};
use diesel::{Queryable};
use diesel_as_jsonb::AsJsonb;
use kernel::{
    db::schema::phaser_scans_events,
    events::EventMetadata,
};
use super::{
    ReportTrigger,
    ScanProfile,
    ScanState,
    ScanSchedule,
};


#[derive(Clone, Debug, Deserialize, Insertable, Queryable, Serialize)]
#[table_name = "phaser_scans_events"]
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
    NameUpdatedV1(NameUpdatedV1),
    DescriptionUpdatedV1(DescriptionUpdatedV1),
    ScheduleUpdatedV1(ScheduleUpdatedV1),
    QueuedV1(QueuedV1),
    CompletedV1,
    StartedV1,
    CanceledV1,
    DeletedV1,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreatedV1 {
    pub id: uuid::Uuid,
    pub owner_id: uuid::Uuid,
    pub name: String,
    pub description: String,
    pub profile: ScanProfile,
    pub schedule: ScanSchedule,
    pub targets: Vec<String>,
}

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

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct QueuedV1 {
    pub trigger: ReportTrigger,
    pub report_id: uuid::Uuid,
}


impl eventsourcing::Event for Event {
    type Aggregate = super::Scan;

    fn apply(&self, aggregate: Self::Aggregate) -> Self::Aggregate {
        match self.data {
            // CreatedV1
            EventData::CreatedV1(ref data) => super::Scan{
                id: data.id,
                created_at: self.timestamp,
                updated_at: self.timestamp,
                deleted_at: None,
                version: 0,

                name: data.name.clone(),
                description: data.description.clone(),
                last: None,
                profile: data.profile.clone(),
                schedule: data.schedule.clone(),
                state: ScanState::Waiting,
                targets: data.targets.clone(),

                owner_id: data.owner_id,
            },
            // NameUpdatedV1
            EventData::NameUpdatedV1(ref data) => super::Scan{
                name: data.name.clone(),
                ..aggregate
            },
            // DescriptionUpdatedV1
            EventData::DescriptionUpdatedV1(ref data) => super::Scan{
                description: data.description.clone(),
                ..aggregate
            },
            // ScheduleUpdatedV1
            EventData::ScheduleUpdatedV1(ref data) => super::Scan{
                schedule: data.schedule.clone(),
                ..aggregate
            },
            // QueuedV1
            EventData::QueuedV1(ref data) => super::Scan{
                state: ScanState::Queued,
                ..aggregate
            },
            // CanceledV1
            EventData::CanceledV1 => super::Scan{
                state: ScanState::Waiting,
                ..aggregate
            },
            // StartedV1
            EventData::StartedV1 => super::Scan{
                state: ScanState::Scanning,
                ..aggregate
            },
            // CompletedV1
            EventData::CompletedV1 => super::Scan{
                state: ScanState::Waiting,
                last: Some(self.timestamp),
                ..aggregate
            },
            // DeletedV1
            EventData::DeletedV1 => super::Scan{
                deleted_at: Some(self.timestamp),
                ..aggregate
            },
        }
    }

    fn timestamp(&self) -> chrono::DateTime<chrono::Utc> {
        return self.timestamp;
    }
}
