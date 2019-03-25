use crate::{
    services::account::domain::account,
    services::account as account_service,
    services::common::events::EventMetadata,
    services::account::validators,
    error::KernelError,
};
use diesel::{
    PgConnection,
    r2d2::{PooledConnection, ConnectionManager},
};


#[derive(Clone, Debug)]
pub struct UpdateAvatar {
    pub avatar: Vec<u8>,
    pub metadata: EventMetadata,
}

impl<'a> eventsourcing::Command<'a> for UpdateAvatar {
    type Aggregate = account::Account;
    type Event = account::Event;
    type Context = PooledConnection<ConnectionManager<PgConnection>>;
    type Error = KernelError;
    type NonStoredData = ();

    fn validate(&self, ctx: &Self::Context, _aggregate: &Self::Aggregate) -> Result<(), Self::Error> {
        if self.avatar.len() > account_service::AVATAR_MAX_SIZE {
            return Err(KernelError::Validation("Image size must be inferior or equal to 3MB.".to_string()));
        }

        // TODO: validate avatar image type

        return Ok(());
    }

    // TODO:
    // resize + crop image to account::AVATAR_RESIZE
    // encode to jpeg
    // uplaod to s3
    fn build_event(&self, _ctx: &Self::Context, aggregate: &Self::Aggregate) -> Result<(Self::Event, Self::NonStoredData), Self::Error> {
        let data = account::EventData::AvatarUpdatedV1(account::AvatarUpdatedV1{
            avatar_url: aggregate.avatar_url.clone(),
        });

        return  Ok((account::Event{
            id: uuid::Uuid::new_v4(),
            timestamp: chrono::Utc::now(),
            data,
            aggregate_id: aggregate.id,
            metadata: self.metadata.clone(),
        }, ()));
    }
}
