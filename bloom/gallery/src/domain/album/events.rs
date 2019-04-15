use serde::{Deserialize, Serialize};
use diesel::{Queryable};
use diesel_as_jsonb::AsJsonb;
use kernel::{
    db::schema::drive_files_events,
    events::EventMetadata,
};


#[derive(Clone, Debug, Deserialize, Insertable, Queryable, Serialize)]
#[table_name = "drive_files_events"]
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
    ItemAddedV1(ItemAddedV1),
    ItemRemovedV1(ItemRemovedV1),
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
pub struct ItemAddedV1 {
    pub file_id: uuid::Uuid,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ItemRemovedV1 {
    pub file_id: uuid::Uuid,
}


impl eventsourcing::Event for Event {
    type Aggregate = super::Album;

    fn apply(&self, aggregate: Self::Aggregate) -> Self::Aggregate {
        match self.data {
            // CreatedV1
            EventData::CreatedV1(ref data) => super::Album{
                id: data.id,
                created_at: self.timestamp,
                updated_at: self.timestamp,
                deleted_at: None,
                version: 0,

                name: data.name.clone(),

                owner_id: data.owner_id,
            },
            // RenamedV1
            EventData::RenamedV1(ref data) => super::Album{
                name: data.name.clone(),
                ..aggregate
            },
            // ItemAddedV1
            EventData::ItemAddedV1(_) => super::Album{
                ..aggregate
            },
            // ItemRemovedV1
            EventData::ItemRemovedV1(_) => super::Album{
                ..aggregate
            },
            // DeletedV1
            EventData::DeletedV1 => super::Album{
                deleted_at: Some(self.timestamp),
                ..aggregate
            },
        }
    }

    fn timestamp(&self) -> chrono::DateTime<chrono::Utc> {
        return self.timestamp;
    }
}
