use diesel::Queryable;
use diesel_as_jsonb::AsJsonb;
use eventsourcing::Aggregate;
use kernel::db::schema::bitflow_downloads;
use serde::{Deserialize, Serialize};

#[derive(
    Aggregate,
    AsChangeset,
    Clone,
    Debug,
    Deserialize,
    Identifiable,
    Insertable,
    Queryable,
    Serialize,
)]
#[table_name = "bitflow_downloads"]
#[changeset_options(treat_none_as_null = "true")]
pub struct Download {
    pub id: uuid::Uuid,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub version: i64,

    pub error: Option<String>,
    pub name: String,
    pub progress: i32,
    pub removed_at: Option<chrono::DateTime<chrono::Utc>>,
    pub status: DownloadStatus,
    pub url: DownloadUrl,

    pub owner_id: uuid::Uuid,
}

#[derive(Clone, Debug, Deserialize, DieselEnum, PartialEq, Serialize)]
pub enum DownloadStatus {
    Queued,
    Downloading,
    Stopped,
    Success,
    Failed,
}

#[derive(AsJsonb, Clone, Debug, Deserialize, Serialize)]
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
pub struct CompleteData {
    pub files: Vec<CompleteDataFile>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CompleteDataFile {
    pub bitflow_id: uuid::Uuid,
    pub path: String,
    pub name: String,
    pub size: u64,
    #[serde(rename = "type")]
    pub type_: String,
}

impl Download {
    // create a new, unitialized note
    pub fn new() -> Self {
        let now = chrono::Utc::now();
        return Download {
            id: uuid::Uuid::new_v4(),
            created_at: now,
            updated_at: now,
            version: 0,

            error: None,
            name: String::new(),
            progress: 0,
            removed_at: None,
            status: DownloadStatus::Queued,
            url: DownloadUrl::Http(DownloadUrlHttp {
                url: "".to_string(),
            }),

            owner_id: uuid::Uuid::new_v4(),
        };
    }
}

impl Default for Download {
    fn default() -> Self {
        Self::new()
    }
}
