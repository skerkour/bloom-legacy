#[macro_use]
extern crate diesel;
#[macro_use]
extern crate try_future;

pub mod error;
pub mod events;
pub mod log;
pub use error::KernelError;
pub mod api;
pub mod config;
pub mod db;
pub mod myaccount;
pub mod notifications;
pub mod utils;

pub static ADDRESS_HELLO: &str = "hello@bloom.sh";
pub static ADDRESS_WELCOME: &str = "welcome@bloom.sh";
pub static ADDRESS_NOTIFY: &str = "notify@bloom.sh";
pub static ADDRESS_SECURITY: &str = "security@bloom.sh";
