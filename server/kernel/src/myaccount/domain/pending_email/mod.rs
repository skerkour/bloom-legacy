mod aggregate;
mod commands;
mod events;

pub use aggregate::PendingEmail;
pub use commands::{Create, Delete, Verify};
pub use events::{CreatedV1, Event, EventData, VerificationFailedReason};
