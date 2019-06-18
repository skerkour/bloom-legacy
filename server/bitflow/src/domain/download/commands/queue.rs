use serde::{Serialize, Deserialize};
use diesel::{
    PgConnection,
    r2d2::{PooledConnection, ConnectionManager},
};
use url::Url;
use kernel::{
    KernelError,
    events::EventMetadata,
};
use crate::{
    domain::download,
    validators,
};


#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Queue {
    pub url: String,
    pub owner_id: uuid::Uuid,
    pub metadata: EventMetadata,
}

impl eventsourcing::Command for Queue {
    type Aggregate = download::Download;
    type Event = download::Event;
    type Context = PooledConnection<ConnectionManager<PgConnection>>;
    type Error = KernelError;
    type NonStoredData = ();

    fn validate(&self, _ctx: &Self::Context, _aggregate: &Self::Aggregate) -> Result<(), Self::Error> {
        validators::download_url(&self.url)?;

        return Ok(());
    }

    fn build_event(&self, _ctx: &Self::Context, _aggregate: &Self::Aggregate) -> Result<(Self::Event, Self::NonStoredData), Self::Error> {
        let id = uuid::Uuid::new_v4();

        // todo: correct url type
        let parsed_url = Url::parse(&self.url)?;
        let url = match parsed_url.scheme() {
            "http" | "https" => {
                download::DownloadUrl::Http(download::DownloadUrlHttp{
                    url: self.url.clone(),
                })
            },
            "magnet" => {
                download::DownloadUrl::TorrentMagnet(download::DownloadUrlTorrentMagnet{
                    magnet: self.url.clone(),
                })
            },
            _ => return Err(KernelError::Validation("Url is not valid. It must be a Http(s) or magnet url.".to_string())),
        };

        let event_data = download::EventData::QueuedV1(download::QueuedV1{
            id: id,
            owner_id: self.owner_id,
            name: self.url.clone(),
            url,
        });

        return  Ok((download::Event{
            id: uuid::Uuid::new_v4(),
            timestamp: chrono::Utc::now(),
            data: event_data,
            aggregate_id: id,
            metadata: self.metadata.clone(),
        }, ()));
    }
}
