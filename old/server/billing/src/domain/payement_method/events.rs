use super::{PaymentDetails, PaymentMethod};
use diesel::Queryable;
use diesel_as_jsonb::AsJsonb;
use kernel::{db::schema::billing_payment_methods_events, events::EventMetadata};
use serde::{Deserialize, Serialize};

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
    SetAsDefaultV1,
    UnsetAsDefaultV1,
    RemovedV1,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AddedV1 {
    pub id: uuid::Uuid,
    pub details: PaymentDetails,
    pub is_default: bool,
    pub billing_profile_id: uuid::Uuid,
}

impl eventsourcing::Event for Event {
    type Aggregate = PaymentMethod;

    fn apply(&self, aggregate: Self::Aggregate) -> Self::Aggregate {
        match self.data {
            // AddedV1
            EventData::AddedV1(ref data) => Self::Aggregate {
                id: data.id,
                created_at: self.timestamp,
                updated_at: self.timestamp,
                version: 0,
                details: data.details.clone(),
                is_default: data.is_default,
                billing_profile_id: data.billing_profile_id,
            },
            // RemovedV1
            EventData::RemovedV1 => aggregate,
            // SetAsDefaultV1
            EventData::SetAsDefaultV1 => Self::Aggregate {
                is_default: true,
                ..aggregate
            },
            // UnsetAsDefaultV1
            EventData::UnsetAsDefaultV1 => Self::Aggregate {
                is_default: false,
                ..aggregate
            },
        }
    }

    fn timestamp(&self) -> chrono::DateTime<chrono::Utc> {
        return self.timestamp;
    }
}
