use serde::{Deserialize, Serialize};
use diesel::{Queryable};
use diesel_as_jsonb::AsJsonb;
use crate::{
    db::schema::billing_payment_methods_events,
    events::EventMetadata,
};
use super::{
    PaymentDetails,
    PaymentMethod,
};


#[derive(Clone, Debug, Deserialize, Insertable, Queryable, Serialize)]
#[table_name = "billing_payment_methods_events"]
pub struct Event {
    pub id: uuid::Uuid,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub data: EventData,
    pub aggregate_id: uuid::Uuid,
    pub metadata: EventMetadata,
}

#[derive(AsJsonb, Clone, Debug, Deserialize, Serialize)]
pub enum EventData {
    AddedV1(AddedV1),
    RemovedV1,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AddedV1 {
    pub id: uuid::Uuid,
    pub details: PaymentDetails,
    pub account_id: uuid::Uuid,
}


impl eventsourcing::Event for Event {
    type Aggregate = PaymentMethod;

    fn apply(&self, aggregate: Self::Aggregate) -> Self::Aggregate {
        match self.data {
            // AddedV1
            EventData::AddedV1(ref data) => Self::Aggregate{
                id: data.id,
                created_at: self.timestamp,
                updated_at: self.timestamp,
                deleted_at: None,
                version: 0,
                details: data.details.clone(),
                account_id: data.account_id,
            },
            // RemovedV1
            EventData::RemovedV1 => Self::Aggregate{
                deleted_at: Some(self.timestamp),
                ..aggregate
            },
        }
    }

    fn timestamp(&self) -> chrono::DateTime<chrono::Utc> {
        return self.timestamp;
    }
}
