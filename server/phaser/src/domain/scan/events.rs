use serde::{Deserialize, Serialize};
use diesel::{Queryable};
use diesel_as_jsonb::AsJsonb;
use kernel::{
    db::schema::phaser_scans_events,
    events::EventMetadata,
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
}
