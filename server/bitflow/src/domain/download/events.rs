use diesel::Queryable;
use diesel_as_jsonb::AsJsonb;
use kernel::{db::schema::bitflow_downloads_events, events::EventMetadata};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Insertable, Queryable, Serialize)]
#[table_name = "bitflow_downloads_events"]
pub struct Event {
    pub id: uuid::Uuid,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub data: EventData,
    pub aggregate_id: uuid::Uuid,
    pub metadata: EventMetadata,
}

#[derive(AsJsonb, Clone, Debug, Deserialize, Serialize)]
pub enum EventData {
    QueuedV1(QueuedV1),
    StartedV1,
    ProgressUpdatedV1(ProgressUpdatedV1),
    NameUpdatedV1(NameUpdatedV1),
    CompletedV1(CompletedV1),
    StoppedV1,
    FailedV1(FailedV1),
    RemovedV1,
    DeletedV1,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct QueuedV1 {
    pub id: uuid::Uuid,
    pub owner_id: uuid::Uuid,
    pub name: String,
    pub url: super::DownloadUrl,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct NameUpdatedV1 {
    pub name: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ProgressUpdatedV1 {
    pub progress: u32,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct FailedV1 {
    pub error: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CompletedV1 {
    pub files: Vec<uuid::Uuid>,
}

impl eventsourcing::Event for Event {
    type Aggregate = super::Download;

    fn apply(&self, aggregate: Self::Aggregate) -> Self::Aggregate {
        match self.data {
            EventData::QueuedV1(ref data) => super::Download {
                id: data.id,
                created_at: self.timestamp,
                updated_at: self.timestamp,
                deleted_at: None,
                version: 0,

                name: data.name.clone(),
                url: data.url.clone(),
                status: super::DownloadStatus::Queued,
                progress: 0,
                removed_at: None,
                error: None,

                owner_id: data.owner_id,
            },
            // StartedV1
            EventData::StartedV1 => super::Download {
                status: super::DownloadStatus::Downloading,
                ..aggregate
            },
            // ProgressUpdatedV1
            EventData::ProgressUpdatedV1(ref data) => super::Download {
                progress: data.progress as i32,
                ..aggregate
            },
            // NameUpdatedV1
            EventData::NameUpdatedV1(ref data) => super::Download {
                name: data.name.clone(),
                ..aggregate
            },
            // CompletedV1
            EventData::CompletedV1(_) => super::Download {
                status: super::DownloadStatus::Success,
                progress: 100,
                ..aggregate
            },
            // StoppedV1
            EventData::StoppedV1 => super::Download {
                status: super::DownloadStatus::Stopped,
                ..aggregate
            },
            // FailedV1
            EventData::FailedV1(ref data) => super::Download {
                error: Some(data.error.clone()),
                status: super::DownloadStatus::Failed,
                ..aggregate
            },
            // RemovedV1
            EventData::RemovedV1 => {
                let status = if aggregate.status == super::DownloadStatus::Queued
                    || aggregate.status == super::DownloadStatus::Downloading
                {
                    super::DownloadStatus::Stopped
                } else {
                    aggregate.status
                };
                super::Download {
                    removed_at: Some(self.timestamp),
                    status,
                    ..aggregate
                }
            }
            // DeletedV1
            EventData::DeletedV1 => super::Download {
                deleted_at: Some(self.timestamp),
                ..aggregate
            },
        }
    }

    fn timestamp(&self) -> chrono::DateTime<chrono::Utc> {
        return self.timestamp;
    }
}
