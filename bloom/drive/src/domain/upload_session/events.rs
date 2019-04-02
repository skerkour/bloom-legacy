use serde::{Deserialize, Serialize};
use diesel::{Queryable};
use diesel_as_jsonb::AsJsonb;
use kernel::{
    db::schema::drive_upload_sessions_events,
    events::EventMetadata,
};


#[derive(Clone, Debug, Deserialize, Insertable, Queryable, Serialize)]
#[table_name = "drive_upload_sessions_events"]
pub struct Event {
    pub id: uuid::Uuid,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub data: EventData,
    pub aggregate_id: uuid::Uuid,
    pub metadata: EventMetadata,
}

#[derive(AsJsonb, Clone, Debug, Deserialize, Serialize)]
pub enum EventData {
    StartedV1(StartedV1),
    CompletedV1(CompletedV1),
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct StartedV1 {
    pub id: uuid::Uuid,
    pub file_name: String,
    pub file_id: uuid::Uuid,
    pub parent_id: Option<uuid::Uuid>,
    pub owner_id: uuid::Uuid,
    pub presigned_url: String,
}


#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CompletedV1 {
    pub size: i64,
    #[serde(rename = "type")]
    pub type_: String,
}


impl eventsourcing::Event for Event {
    type Aggregate = super::UploadSession;

    fn apply(&self, aggregate: Self::Aggregate) -> Self::Aggregate {
        match self.data {
            // StartedV1
            EventData::StartedV1(ref data) => super::UploadSession{
                id: data.id,
                created_at: self.timestamp,
                updated_at: self.timestamp,
                deleted_at: None,
                version: 0,

                file_name: data.file_name.clone(),
                file_id: data.file_id,
                parent_id: data.parent_id,
                presigned_url: data.presigned_url.clone(),
                size: 0,
                type_: String::new(),

                owner_id: data.owner_id,
            },
            // CompletedV1
            EventData::CompletedV1(ref data) => super::UploadSession{
                deleted_at: self.timestamp,
                size: data.size,
                type_: data.type_.clone(),
                ..aggregate
            },
        }
    }

    fn timestamp(&self) -> chrono::DateTime<chrono::Utc> {
        return self.timestamp;
    }
}
