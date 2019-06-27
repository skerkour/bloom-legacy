use diesel::Queryable;
use diesel_as_jsonb::AsJsonb;
use kernel::{db::schema::calendar_events_events, events::EventMetadata};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Insertable, Queryable, Serialize)]
#[table_name = "calendar_events_events"]
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
    TitleUpdatedV1(TitleUpdatedV1),
    DescriptionUpdatedV1(DescriptionUpdatedV1),
    StartAtUpdatedV1(StartAtUpdatedV1),
    EndAtUpdatedV1(EndAtUpdatedV1),
    DeletedV1,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreatedV1 {
    pub id: uuid::Uuid,
    pub title: String,
    pub description: String,
    pub start_at: chrono::DateTime<chrono::Utc>,
    pub end_at: chrono::DateTime<chrono::Utc>,
    pub owner_id: uuid::Uuid,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TitleUpdatedV1 {
    pub title: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DescriptionUpdatedV1 {
    pub description: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct StartAtUpdatedV1 {
    pub start_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EndAtUpdatedV1 {
    pub end_at: chrono::DateTime<chrono::Utc>,
}

impl eventsourcing::Event for Event {
    type Aggregate = super::CalendarEvent;

    fn apply(&self, aggregate: Self::Aggregate) -> Self::Aggregate {
        match self.data {
            // CreatedV1
            EventData::CreatedV1(ref data) => super::CalendarEvent {
                id: data.id,
                created_at: self.timestamp,
                updated_at: self.timestamp,
                deleted_at: None,
                version: 0,

                title: data.title.clone(),
                description: data.description.clone(),
                start_at: data.start_at,
                end_at: data.end_at,
                owner_id: data.owner_id,
            },
            // TitleUpdatedV1
            EventData::TitleUpdatedV1(ref data) => super::CalendarEvent {
                title: data.title.clone(),
                ..aggregate
            },
            // DescriptionUpdatedV1
            EventData::DescriptionUpdatedV1(ref data) => super::CalendarEvent {
                description: data.description.clone(),
                ..aggregate
            },
            // StartAtUpdatedV1
            EventData::StartAtUpdatedV1(ref data) => super::CalendarEvent {
                start_at: data.start_at,
                ..aggregate
            },
            // EndAtUpdatedV1
            EventData::EndAtUpdatedV1(ref data) => super::CalendarEvent {
                end_at: data.end_at,
                ..aggregate
            },
            // DeletedV1
            EventData::DeletedV1 => super::CalendarEvent {
                deleted_at: Some(self.timestamp),
                ..aggregate
            },
        }
    }

    fn timestamp(&self) -> chrono::DateTime<chrono::Utc> {
        return self.timestamp;
    }
}
