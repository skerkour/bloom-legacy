use serde::{Deserialize, Serialize};
use diesel::{Queryable};
use diesel_as_jsonb::AsJsonb;
use kernel::{
    db::schema::music_playlists_events,
    events::EventMetadata,
};


#[derive(Clone, Debug, Deserialize, Insertable, Queryable, Serialize)]
#[table_name = "music_playlists_events"]
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
    RenamedV1(RenamedV1),
    FilesAddedV1(FilesAddedV1),
    FilesRemovedV1(FilesRemovedV1),
    DeletedV1,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreatedV1 {
    pub id: uuid::Uuid,
    pub name: String,
    pub owner_id: uuid::Uuid,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RenamedV1 {
    pub name: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct FilesAddedV1 {
    pub files: Vec<uuid::Uuid>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct FilesRemovedV1 {
    pub files: Vec<uuid::Uuid>,
}


impl eventsourcing::Event for Event {
    type Aggregate = super::Playlist;

    fn apply(&self, aggregate: Self::Aggregate) -> Self::Aggregate {
        match self.data {
            // CreatedV1
            EventData::CreatedV1(ref data) => super::Playlist{
                id: data.id,
                created_at: self.timestamp,
                updated_at: self.timestamp,
                deleted_at: None,
                version: 0,

                name: data.name.clone(),

                owner_id: data.owner_id,
            },
            // RenamedV1
            EventData::RenamedV1(ref data) => super::Playlist{
                name: data.name.clone(),
                ..aggregate
            },
            // FilesAddedV1
            EventData::FilesAddedV1(_) => super::Playlist{
                ..aggregate
            },
            // FilesRemovedV1
            EventData::FilesRemovedV1(_) => super::Playlist{
                ..aggregate
            },
            // DeletedV1
            EventData::DeletedV1 => super::Playlist{
                deleted_at: Some(self.timestamp),
                ..aggregate
            },
        }
    }

    fn timestamp(&self) -> chrono::DateTime<chrono::Utc> {
        return self.timestamp;
    }
}
