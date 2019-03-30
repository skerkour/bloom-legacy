use serde::{Deserialize, Serialize};
use diesel::{Queryable};
use diesel_as_jsonb::AsJsonb;
use crate::{
    db::schema::notes_notes_events,
    services::common::events::EventMetadata,
};
use std::string::ToString;



#[derive(Clone, Debug, Deserialize, Insertable, Queryable, Serialize)]
#[table_name = "notes_notes_events"]
pub struct Event {
    pub id: uuid::Uuid,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub data: EventData,
    pub aggregate_id: uuid::Uuid,
    pub metadata: EventMetadata, // TODO: change
}

#[derive(AsJsonb, Clone, Debug, Deserialize, Serialize)]
pub enum EventData {
    CreatedV1(CreatedV1),
    ArchivedV1,
    DeletedV1,
    RemovedV1,
    RestoredV1,
    UnarchivedV1,
    BodyUpdatedV1(BodyUpdatedV1),
    TitleUpdatedV1(TitleUpdatedV1),
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreatedV1 {
    pub id: uuid::Uuid,
    pub title: String,
    pub body: String,
    pub owner_id: uuid::Uuid,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BodyUpdatedV1 {
    pub body: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TitleUpdatedV1 {
    pub title: String,
}


impl eventsourcing::Event for Event {
    type Aggregate = super::Note;

    fn apply(&self, aggregate: Self::Aggregate) -> Self::Aggregate {
        match self.data {
            // CreatedV1
            EventData::CreatedV1(ref data) => super::Note{
                id: data.id,
                created_at: self.timestamp,
                updated_at: self.timestamp,
                deleted_at: None,
                version: 0,

                archived_at: None,
                body: data.body.clone(),
                removed_at: None,
                title: data.title.clone(),

                owner_id: data.owner_id,
            },
            // ArchivedV1
            EventData::ArchivedV1 => super::Note{
                archived_at: Some(self.timestamp),
                ..aggregate.clone()
            },
            // DeletedV1
            EventData::DeletedV1 => super::Note{
                deleted_at: Some(self.timestamp),
                removed_at: None,
                ..aggregate.clone()
            },
            // RemovedV1
            EventData::RemovedV1 => super::Note{
                removed_at: Some(self.timestamp),
                ..aggregate.clone()
            },
            // RestoredV1
            EventData::RestoredV1 => super::Note{
                removed_at: None,
                ..aggregate.clone()
            },
            // UnarchivedV1
            EventData::UnarchivedV1 => super::Note{
                archived_at: None,
                ..aggregate.clone()
            },
            // BodyUpdatedV1
            EventData::BodyUpdatedV1(ref data) => super::Note{
                body: data.body.clone(),
                ..aggregate.clone()
            },
            // TitleUpdatedV1
            EventData::TitleUpdatedV1(ref data) => super::Note{
                title: data.title.clone(),
                ..aggregate.clone()
            },
        }
    }

    fn timestamp(&self) -> chrono::DateTime<chrono::Utc> {
        return self.timestamp;
    }
}
