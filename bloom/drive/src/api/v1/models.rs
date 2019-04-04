use serde::{Serialize, Deserialize};
use uuid::Uuid;


#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct StartUploadSessionBody {
    pub file_name: String,
    pub parent_id: Option<Uuid>,
}


#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct StartUploadSessionResponse {
    pub id: Uuid,
    pub presigned_url: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CompleteUploadSessionBody {
    pub upload_session_id: Uuid,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PorfileBody {
    pub used_space: i64,
    pub total_space: i64,
    pub home: Uuid,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct FileBody {
    pub id: Uuid,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub name: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub size: i64,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct FolderBody {
    pub id: Uuid,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub path: Vec<String>,
    pub name: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub size: i64,
    pub children: Vec<FileBody>,
}
