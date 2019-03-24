mod commands;
mod events;
mod aggregate;


pub use aggregate::Account;
pub use commands::{
    Create,
    UpdateFirstName,
    UpdateLastName,
    UpdatePassword,
    UpdateEmail,
    FailSignIn,
};
pub use events::{
    Event,
    EventData,
    CreatedV1,
    FirstNameUpdatedV1,
    LastNameUpdatedV1,
    PasswordUpdatedV1,
    EmailUpdatedV1,
};
