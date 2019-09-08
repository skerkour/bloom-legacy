use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateNoteBody {
    pub title: String,
    pub body: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct NoteResponse {
    pub id: uuid::Uuid,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub archived_at: Option<chrono::DateTime<chrono::Utc>>,
    pub removed_at: Option<chrono::DateTime<chrono::Utc>>,
    pub title: String,
    pub body: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UpdateNote {
    pub title: Option<String>,
    pub body: Option<String>,
}
