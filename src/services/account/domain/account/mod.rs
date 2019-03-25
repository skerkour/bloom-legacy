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
    UpdateAvatar,
};
pub use events::{
    Event,
    EventData,
    CreatedV1,
    FirstNameUpdatedV1,
    LastNameUpdatedV1,
    PasswordUpdatedV1,
    EmailUpdatedV1,
    AvatarUpdatedV1,
};


use diesel::{
    PgConnection,
    r2d2::{PooledConnection, ConnectionManager},
};
use rusoto_s3::S3Client;

pub struct UpdateAvatarCtx<'a> {
    pub conn: &'a PooledConnection<ConnectionManager<PgConnection>>,
    pub s3_client: S3Client,
}
