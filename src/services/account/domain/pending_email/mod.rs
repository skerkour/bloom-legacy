use serde::{Serialize, Deserialize};
use diesel::{Queryable};


#[derive(Clone, Debug, Deserialize, Queryable, Serialize)]
pub struct PendingEmail {
    pub id: uuid::Uuid,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub deleted_at: Option<chrono::DateTime<chrono::Utc>>,
    pub version: i64,

    pub email: String,
    pub token: String, // hashed token

    pub account_id: uuid::Uuid,
}
