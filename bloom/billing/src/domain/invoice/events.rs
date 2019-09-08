use serde::{Deserialize, Serialize};
use diesel::{Queryable};
use diesel_as_jsonb::AsJsonb;
use crate::{
    db::schema::billing_invoices_events,
    events::EventMetadata,
};
use super::{
    Invoice,
};


#[derive(Clone, Debug, Deserialize, Insertable, Queryable, Serialize)]
#[table_name = "billing_invoices_events"]
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
    type Aggregate = Invoice;

    fn apply(&self, aggregate: Self::Aggregate) -> Self::Aggregate {
        match self.data {
        }
    }

    fn timestamp(&self) -> chrono::DateTime<chrono::Utc> {
        return self.timestamp;
    }
}
