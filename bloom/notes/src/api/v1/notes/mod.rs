mod delete;
mod get;
mod post;
mod put;

pub use delete::delete;
pub use get::get;
pub use post::post;
pub use put::put;
pub mod archive;
pub mod remove;
pub mod restore;
pub mod unarchive;
