#[macro_use]
extern crate diesel;
#[macro_use]
extern crate try_future;

pub mod log;
pub mod events;
pub mod error;
pub use error::KernelError;
pub mod config;
pub mod utils;
pub mod notifications;
pub mod db;
pub mod api;
pub mod myaccount;


pub static ADDRESS_HELLO: &str = "hello@bloom.sh";
pub static ADDRESS_WELCOME: &str = "welcome@bloom.sh";
pub static ADDRESS_NOTIFY: &str = "notify@bloom.sh";
pub static ADDRESS_SECURITY: &str = "security@bloom.sh";
