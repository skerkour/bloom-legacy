use diesel_as_jsonb::AsJsonb;
use kernel::db::schema::billing_payment_methods;
use serde::{Deserialize, Serialize};

#[derive(AsChangeset, Clone, Debug, Deserialize, Identifiable, Insertable, Queryable, Serialize)]
#[table_name = "billing_payment_methods"]
#[changeset_options(treat_none_as_null = "true")]
pub struct PaymentMethod {
    pub id: uuid::Uuid,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub version: i64,

    pub details: PaymentDetails,
    pub is_default: bool,

    pub billing_profile_id: uuid::Uuid,
}

impl PaymentMethod {
    // create a new, unitialized PaymentMethod
    pub fn new() -> Self {
        let now = chrono::Utc::now();
        return Self {
            id: uuid::Uuid::new_v4(),
            created_at: now,
            updated_at: now,
            version: 0,

            details: PaymentDetails::None,
            is_default: false,

            billing_profile_id: uuid::Uuid::new_v4(),
        };
    }
}

impl eventsourcing::Aggregate for PaymentMethod {
    fn increment_version(&mut self) {
        self.version += 1;
    }

    fn update_updated_at(&mut self, timestamp: chrono::DateTime<chrono::Utc>) {
        self.updated_at = timestamp;
    }
}

impl Default for PaymentMethod {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(AsJsonb, Clone, Debug, Deserialize, Serialize)]
pub enum PaymentDetails {
    None,
    StripeCardV1(StripeCardV1),
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct StripeCardV1 {
    pub stripe_token_card_id: String,
    pub last4: String,
}
