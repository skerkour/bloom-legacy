use serde::{Serialize, Deserialize};
use uuid::Uuid;
use crate::domain;


#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct StartUploadBody {
    pub file_name: String,
    pub parent_id: Option<Uuid>,
}


#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct StartUploadResponse {
    pub id: Uuid,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CompleteUploadBody {
    pub upload_id: Uuid,
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
    pub path: Vec<domain::file::FolderPath>,
    pub name: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub size: i64,
    pub files: Vec<FileBody>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct FileUrl {
    pub url: String,
}


#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateFolderBody {
    pub name: String,
    pub parent_id: Uuid,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MoveBody {
    pub files: Vec<Uuid>,
    pub to: Uuid,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TrashFilesBody {
    pub files: Vec<Uuid>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RestoreFilesBody {
    pub files: Vec<Uuid>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DeleteFilesBody {
    pub files: Vec<Uuid>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CopyFilesBody {
    pub to: Uuid,
    pub files: Vec<Uuid>,
}

