use crate::downloader::download;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ApiResponse {
    pub error: Option<String>,
    pub data: Option<JobResponse>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct JobResponse {
    pub id: uuid::Uuid,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub name: String,
    pub url: download::DownloadUrl,
    pub max_size: u64,
    pub error: Option<String>,
    pub progress: i32,
}

impl Into<download::Download> for JobResponse {
    fn into(self) -> download::Download {
        return download::Download {
            id: self.id,
            error: self.error,
            name: self.name,
            progress: self.progress,
            url: self.url,
            max_size: self.max_size,
        };
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum DownloadStatus {
    Queued,
    Downloading,
    Stopped,
    Completed,
    Failed,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum DownloadUrl {
    Http(DownloadUrlHttp),
    TorrentMagnet(DownloadUrlTorrentMagnet),
}

impl Into<download::DownloadUrl> for DownloadUrl {
    fn into(self) -> download::DownloadUrl {
        match self {
            DownloadUrl::Http(data) => download::DownloadUrl::Http(data.into()),
            DownloadUrl::TorrentMagnet(data) => download::DownloadUrl::TorrentMagnet(data.into()),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DownloadUrlHttp {
    pub url: String,
}

impl Into<download::DownloadUrlHttp> for DownloadUrlHttp {
    fn into(self) -> download::DownloadUrlHttp {
        return download::DownloadUrlHttp { url: self.url };
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DownloadUrlTorrentMagnet {
    pub magnet: String,
}

impl Into<download::DownloadUrlTorrentMagnet> for DownloadUrlTorrentMagnet {
    fn into(self) -> download::DownloadUrlTorrentMagnet {
        return download::DownloadUrlTorrentMagnet {
            magnet: self.magnet,
        };
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct FailDownloadBody {
    pub error: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CompleteBody {
    pub files: Vec<download::DownloadedFile>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UpdateNameBody {
    pub name: String,
}
