use serde::{Serialize, Deserialize};
use diesel_as_jsonb::AsJsonb;
use crate::{
    db::schema::billing_payment_methods,
};


#[derive(AsChangeset, Clone, Debug, Deserialize, Identifiable, Insertable, Queryable, Serialize)]
#[table_name = "billing_payment_methods"]
#[changeset_options(treat_none_as_null = "true")]
pub struct PaymentMethod {
    pub id: uuid::Uuid,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub deleted_at: Option<chrono::DateTime<chrono::Utc>>,
    pub version: i64,

    pub details: PaymentDetails,

    pub account_id: uuid::Uuid,
}


impl PaymentMethod {
    // create a new, unitialized PaymentMethod
    pub fn new() -> Self {
        let now = chrono::Utc::now();
        return Self{
            id: uuid::Uuid::new_v4(),
            created_at: now,
            updated_at: now,
            deleted_at: None,
            version: 0,

            details: PaymentDetails::None,

            account_id: uuid::Uuid::new_v4(),
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

#[derive(AsJsonb, Clone, Debug, Deserialize, Serialize)]
pub enum PaymentDetails {
    None,
    StripeCard,
}
