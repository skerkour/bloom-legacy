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
}


impl eventsourcing::Event for Event {
    type Aggregate = super::Download;

    fn apply(&self, aggregate: Self::Aggregate) -> Self::Aggregate {
        // match self.data {
        //     // CreatedV1
        //     EventData::CreatedV1(ref data) => super::Download{
        //     }
        // }
        aggregate
    }

    fn timestamp(&self) -> chrono::DateTime<chrono::Utc> {
        return self.timestamp;
    }
}
