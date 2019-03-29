mod post;
mod get;
mod put;
mod delete;


pub use post::post;
pub use get::get;
pub use put::put;
pub use delete::delete;
pub mod archive;
pub mod remove;
pub mod restore;
pub mod unarchive;
