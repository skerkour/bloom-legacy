mod aggregate;
mod events;
mod commands;


pub use aggregate::{
    Download,
    DownloadState,
    DownloadUrl,
    DownloadUrlHttp,
    DownloadUrlTorrentMagnet,
};
pub use commands::{
    Delete,
    Queue,
};
pub use events::{
    Event,
    EventData,
    QueuedV1,
    ProgressUpdatedV1,
    NameUpdatedV1,
    FailedV1,
};
