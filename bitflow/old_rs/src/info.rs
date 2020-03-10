use clap;
use std::env::consts;

pub static NAME: &str = clap::crate_name!();
pub static VERSION: &str = clap::crate_version!();
pub static AUTHOR: &str = clap::crate_authors!();
pub static DESCRPITION: &str = clap::crate_description!();
pub static OS: &str = consts::OS;
pub static ARCH: &str = consts::ARCH;
