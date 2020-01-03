use crate::{dart_date_format, dart_date_format_opt};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Note {
    pub id: String,
    #[serde(with = "dart_date_format")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    #[serde(with = "dart_date_format")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    #[serde(with = "dart_date_format_opt")]
    pub archived_at: Option<chrono::DateTime<chrono::Utc>>,
    pub title: String,
    pub body: String,
    pub color: i64,
    pub is_pinned: bool,
}
