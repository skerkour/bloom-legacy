mod commands;
mod events;
mod aggregate;


pub use aggregate::Note;
pub use commands::{
    Create,
    Archive,
    Delete,
    Remove,
    Restore,
    Unarchive,
    UpdateBody,
    UpdateTitle,
};
pub use events::{
    Event,
    EventData,
    CreatedV1,
    BodyUpdatedV1,
    TitleUpdatedV1,
};
