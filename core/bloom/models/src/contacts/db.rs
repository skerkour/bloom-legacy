use crate::{dart_date_format, dart_date_format_opt};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Conatct {
    pub id: String,
    #[serde(with = "dart_date_format")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    #[serde(with = "dart_date_format")]
    pub updated_at: chrono::DateTime<chrono::Utc>,

    pub first_name: String,
    pub last_name: String,
    pub notes: String,

    pub addresses: serde_json::Value,
    #[serde(with = "dart_date_format_opt")]
    pub birthday: Option<chrono::DateTime<chrono::Utc>>,
    pub organizations: serde_json::Value,
    pub emails: serde_json::Value,
    pub phones: serde_json::Value,
    pub websites: serde_json::Value,
    pub device_id: String,
}
