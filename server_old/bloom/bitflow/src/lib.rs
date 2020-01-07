#![feature(ip)]

#[macro_use]
extern crate diesel;

#[macro_use]
extern crate diesel_enum_derive;

pub mod api;
pub mod controllers;
pub mod domain;
pub mod reactors;

pub const MAX_DOWNLOAD_SIZE: u64 = 8_000_000_000; // 8 GB
