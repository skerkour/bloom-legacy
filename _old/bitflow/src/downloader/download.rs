use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Download {
    pub id: uuid::Uuid,
    pub error: Option<String>,
    pub name: String,
    pub progress: i32,
    pub url: DownloadUrl,
    pub max_size: u64,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum DownloadUrl {
    Http(DownloadUrlHttp),
    TorrentMagnet(DownloadUrlTorrentMagnet),
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DownloadUrlHttp {
    pub url: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DownloadUrlTorrentMagnet {
    pub magnet: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Downloaded {
    pub download: Download,
    pub files: Vec<DownloadedFile>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DownloadedFile {
    pub bitflow_id: uuid::Uuid,
    pub name: String,
    pub path: String,
    pub size: u64,
    #[serde(rename = "type")]
    pub type_: String,
}
