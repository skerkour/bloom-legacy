#[macro_use]
extern crate diesel;

pub mod api;
pub mod domain;
pub mod controllers;
pub mod validators;

pub const DEFAULT_AVAILABLE_SPACE: i64 = 30 * 1024 * 1024 * 1024; // 30GB
