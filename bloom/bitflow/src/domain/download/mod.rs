mod aggregate;
mod commands;

pub mod validators;
pub use aggregate::{
    CompleteData, CompleteDataFile, Download, DownloadStatus, DownloadUrl, DownloadUrlHttp,
    DownloadUrlTorrentMagnet,
};
pub use commands::{
    complete::{Complete, Completed},
    delete::{Delete, Deleted},
    fail::{Fail, Failed},
    queue::{Queue, Queued},
    remove::{Remove, Removed},
    start::{Start, Started},
    update_name::{NameUpdated, UpdateName},
    update_progress::{ProgressUpdated, UpdateProgress},
};
