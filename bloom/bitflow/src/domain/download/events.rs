use serde::{Deserialize, Serialize};
use diesel::{Queryable};
use diesel_as_jsonb::AsJsonb;
use kernel::{
    db::schema::bitflow_downloads_events,
    events::EventMetadata,
};


#[derive(Clone, Debug, Deserialize, Insertable, Queryable, Serialize)]
#[table_name = "bitflow_downloads_events"]
pub struct Event {
    pub id: uuid::Uuid,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub data: EventData,
    pub aggregate_id: uuid::Uuid,
    pub metadata: EventMetadata,
}

#[derive(AsJsonb, Clone, Debug, Deserialize, Serialize)]
pub enum EventData {
    QueuedV1,
    StartedV1,
    ProgressUpdatedV1(ProgressUpdatedV1),
    NameUpdatedV1(NameUpdatedV1),
    CompletedV1,
    StoppedV1,
    FailedV1,
    RemovedV1,
    DeletedV1,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct NameUpdatedV1 {
    pub name: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ProgressUpdatedV1 {
    pub progress: u32,
}


impl eventsourcing::Event for Event {
    type Aggregate = super::Download;

    fn apply(&self, aggregate: Self::Aggregate) -> Self::Aggregate {
        match self.data {
            // ProgressUpdatedV1
            EventData::ProgressUpdatedV1(ref data) => super::Download{
                progress: data.progress as i32,
                ..aggregate
            },
            // NameUpdatedV1
            EventData::NameUpdatedV1(ref data) => super::Download{
                name: data.name.clone(),
                ..aggregate
            },
            _ => aggregate,
        }
    }

    fn timestamp(&self) -> chrono::DateTime<chrono::Utc> {
        return self.timestamp;
    }
}
