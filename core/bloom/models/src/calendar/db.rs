use crate::dart_date_format;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Event {
    pub id: String,
    #[serde(with = "dart_date_format")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    #[serde(with = "dart_date_format")]
    pub updated_at: chrono::DateTime<chrono::Utc>,

    pub title: String,
    pub description: String,
    #[serde(with = "dart_date_format")]
    pub start_at: chrono::DateTime<chrono::Utc>,
    #[serde(with = "dart_date_format")]
    pub end_at: chrono::DateTime<chrono::Utc>,
}
