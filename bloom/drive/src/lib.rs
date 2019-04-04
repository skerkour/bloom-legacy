#[macro_use]
extern crate diesel;

pub mod api;
pub mod domain;
pub mod controllers;
pub mod validators;
pub mod reactors;

pub const DEFAULT_AVAILABLE_SPACE: i64 = 30 * 1024 * 1024 * 1024; // 30GB
pub const DEFAULT_FOLDERS: [&'static str; 8] = [
    "Books",
    "Downloads",
    "Documents",
    "Games",
    "Music",
    "Pictures",
    "Videos",
    "Projects",
];
