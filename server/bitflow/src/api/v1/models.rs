use serde::{Serialize, Deserialize};
use crate::domain::download;


#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DownloadResponse {
    pub id: uuid::Uuid,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub error: Option<String>,
    pub name: String,
    pub progress: i32,
    pub removed_at: Option<chrono::DateTime<chrono::Utc>>,
    pub status: download::DownloadStatus,
    pub url: download::DownloadUrl,
}

impl From<download::Download> for DownloadResponse {
    fn from(download: download::Download) -> Self {
        DownloadResponse{
            id: download.id,
            created_at: download.created_at,
            error: download.error,
            name: download.name,
            progress: download.progress,
            removed_at: download.removed_at,
            status: download.status,
            url: download.url,
        }
    }
}


#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct QueueDownloadBody {
    pub url: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoveDownloadsBody {
    pub downloads: Vec<uuid::Uuid>,
}


#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UpdateDownloadBody {
    pub name: Option<String>,
    pub progress: Option<u32>,
}


#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct FailDownloadBody {
    pub error: String,
}
