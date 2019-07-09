mod aggregate;
mod commands;

pub mod validators;
pub use aggregate::Note;
pub use commands::{
    archive::{Archive, Archived},
    create::{Create, Created},
    delete::{Delete, Deleted},
    remove::{Remove, Removed},
    restore::{Restore, Restored},
    unarchive::{Unarchive, Unarchived},
    update_body::{BodyUpdated, UpdateBody},
    update_title::{TitleUpdated, UpdateTitle},
};
