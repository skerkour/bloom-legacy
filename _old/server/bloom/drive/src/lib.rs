#[macro_use]
extern crate diesel;

pub mod api;
pub mod controllers;
pub mod domain;
pub mod reactors;

pub const DEFAULT_AVAILABLE_SPACE: i64 = 30_000_000_000; // 30GB
pub const DEFAULT_FOLDERS: [&str; 8] = [
    "Books",
    "Downloads",
    "Documents",
    "Games",
    "Music",
    "Pictures",
    "Videos",
    "Projects",
];

pub const BLOOM_ROOT_NAME: &str = "__BLOOM_ROOT";
pub const BLOOM_TRASH_NAME: &str = "__BLOOM_TRASH";
pub const FOLDER_TYPE: &str = "application/vnd.bloom.folder";
pub const UPLOAD_MAX_SIZE: u64 = 5_000_000_000; // 5GB
