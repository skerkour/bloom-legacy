#![feature(ip)]

#[macro_use]
extern crate diesel;

#[macro_use]
extern crate diesel_enum_derive;

pub mod api;
pub mod controllers;
pub mod domain;
pub mod models;

pub const REPORT_MAX_SIZE: u64 = 100_000_000; // 100 MB
