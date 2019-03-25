mod commands;
mod events;
mod aggregate;


pub use aggregate::PendingEmail;
pub use commands::{
    Create,
    Verify,
};
pub use events::{
    Event,
    EventData,
    CreatedV1,
    VerificationFailedV1,
};
