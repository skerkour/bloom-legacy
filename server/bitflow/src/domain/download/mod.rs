mod aggregate;
mod commands;
mod events;

pub use aggregate::{
    CompleteData, CompleteDataFile, Download, DownloadStatus, DownloadUrl, DownloadUrlHttp,
    DownloadUrlTorrentMagnet,
};
pub use commands::{Complete, Delete, Fail, Queue, Remove, Start, UpdateName, UpdateProgress};
pub use events::{
    CompletedV1, Event, EventData, FailedV1, NameUpdatedV1, ProgressUpdatedV1, QueuedV1,
};
