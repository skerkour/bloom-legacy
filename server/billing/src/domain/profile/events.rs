use super::Profile;
use diesel::Queryable;
use diesel_as_jsonb::AsJsonb;
use kernel::{db::schema::billing_profiles_events, events::EventMetadata};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Insertable, Queryable, Serialize)]
#[table_name = "billing_profiles_events"]
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
    StripeCustomerIdUpdatedV1(StripeCustomerIdUpdatedV1),
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreatedV1 {
    pub id: uuid::Uuid,
    pub account_id: uuid::Uuid,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct StripeCustomerIdUpdatedV1 {
    pub stripe_customer_id: String,
}

impl eventsourcing::Event for Event {
    type Aggregate = Profile;

    fn apply(&self, aggregate: Self::Aggregate) -> Self::Aggregate {
        match self.data {
            // CreatedV1
            EventData::CreatedV1(ref data) => Self::Aggregate {
                id: data.id,
                created_at: self.timestamp,
                updated_at: self.timestamp,
                version: 0,
                account_id: data.account_id,
                stripe_customer_id: None,
            },
            // StripeCustomerIdUpdatedV1
            EventData::StripeCustomerIdUpdatedV1(ref data) => Self::Aggregate {
                stripe_customer_id: Some(data.stripe_customer_id.clone()),
                ..aggregate
            },
        }
    }

    fn timestamp(&self) -> chrono::DateTime<chrono::Utc> {
        return self.timestamp;
    }
}
