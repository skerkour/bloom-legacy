use crate::domain::download;
use diesel::{
    r2d2::{ConnectionManager, PooledConnection},
    PgConnection,
};
use eventsourcing::{Event, EventTs};
use kernel::KernelError;
use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Queue {
    pub url: String,
    pub owner_id: uuid::Uuid,
}

impl eventsourcing::Command for Queue {
    type Aggregate = download::Download;
    type Event = Queued;
    type Context = PooledConnection<ConnectionManager<PgConnection>>;
    type Error = KernelError;

    fn validate(
        &self,
        _ctx: &Self::Context,
        _aggregate: &Self::Aggregate,
    ) -> Result<(), Self::Error> {
        download::validators::url(&self.url)?;

        return Ok(());
    }

    fn build_event(
        &self,
        _ctx: &Self::Context,
        _aggregate: &Self::Aggregate,
    ) -> Result<Self::Event, Self::Error> {
        // todo: correct url type
        let parsed_url = Url::parse(&self.url)?;
        let url = match parsed_url.scheme() {
            "http" | "https" => download::DownloadUrl::Http(download::DownloadUrlHttp {
                url: self.url.clone(),
            }),
            "magnet" => download::DownloadUrl::TorrentMagnet(download::DownloadUrlTorrentMagnet {
                magnet: self.url.clone(),
            }),
            _ => {
                return Err(KernelError::Validation(
                    "Url is not valid. It must be a Http(s) or magnet url.".to_string(),
                ))
            }
        };

        return Ok(Queued {
            timestamp: chrono::Utc::now(),
            id: uuid::Uuid::new_v4(),
            owner_id: self.owner_id,
            name: self.url.clone(),
            url,
        });
    }
}

// Event
#[derive(Clone, Debug, EventTs)]
pub struct Queued {
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub id: uuid::Uuid,
    pub owner_id: uuid::Uuid,
    pub name: String,
    pub url: download::DownloadUrl,
}

impl Event for Queued {
    type Aggregate = download::Download;

    fn apply(&self, _aggregate: Self::Aggregate) -> Self::Aggregate {
        return Self::Aggregate {
            id: self.id,
            created_at: self.timestamp,
            updated_at: self.timestamp,
            version: 0,
            name: self.name.clone(),
            url: self.url.clone(),
            status: download::DownloadStatus::Queued,
            progress: 0,
            removed_at: None,
            error: None,
            owner_id: self.owner_id,
        };
    }
}
