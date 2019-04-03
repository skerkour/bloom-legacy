use serde::{Deserialize, Serialize};
use diesel::{Queryable};
use diesel_as_jsonb::AsJsonb;
use kernel::{
    db::schema::drive_profiles_events,
    events::EventMetadata,
};


#[derive(Clone, Debug, Deserialize, Insertable, Queryable, Serialize)]
#[table_name = "drive_profiles_events"]
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
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreatedV1 {
    pub id: uuid::Uuid,
    pub home_id: uuid::Uuid,
    pub total_space: i64,
    pub account_id: uuid::Uuid,
}


impl eventsourcing::Event for Event {
    type Aggregate = super::Profile;

    fn apply(&self, aggregate: Self::Aggregate) -> Self::Aggregate {
        match self.data {
            // CreatedV1
            EventData::CreatedV1(ref data) => super::Profile{
                id: data.id,
                created_at: self.timestamp,
                updated_at: self.timestamp,
                deleted_at: None,
                version: 0,

                total_space: data.total_space,
                used_space: 0,

                account_id: data.account_id,
                home_id: data.home_id,
            },
        }
    }

    fn timestamp(&self) -> chrono::DateTime<chrono::Utc> {
        return self.timestamp;
    }
}
