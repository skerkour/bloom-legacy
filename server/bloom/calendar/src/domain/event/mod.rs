mod aggregate;
mod commands;

pub mod validators;
pub use aggregate::CalendarEvent;
pub use commands::{
    create::{Create, Created},
    delete::{Delete, Deleted},
    update_description::{DescriptionUpdated, UpdateDescription},
    update_end_at::{EndAtUpdated, UpdateEndAt},
    update_start_at::{StartAtUpdated, UpdateStartAt},
    update_title::{TitleUpdated, UpdateTitle},
};
