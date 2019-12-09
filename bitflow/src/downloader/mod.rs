#[allow(clippy::module_inception)]
mod downloader;

pub mod download;
pub use downloader::Downloader;
