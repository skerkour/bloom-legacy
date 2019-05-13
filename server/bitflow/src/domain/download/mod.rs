mod aggregate;
mod events;
mod commands;


pub use aggregate::{
    Download,
    DownloadStatus,
    DownloadUrl,
    DownloadUrlHttp,
    DownloadUrlTorrentMagnet,
    CompleteData,
    CompleteDataFile,
};
pub use commands::{
    Delete,
    Queue,
    Remove,
    Start,
    UpdateName,
    UpdateProgress,
    Complete,
    Fail,
};
pub use events::{
    Event,
    EventData,
    QueuedV1,
    ProgressUpdatedV1,
    NameUpdatedV1,
    FailedV1,
    CompletedV1,
};
