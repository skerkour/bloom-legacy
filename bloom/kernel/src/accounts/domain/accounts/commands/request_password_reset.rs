use diesel::{
    PgConnection,
    r2d2::{PooledConnection, ConnectionManager},
};
use rand::Rng;
use crate::{
    accounts::domain::accounts,
    events::EventMetadata,
    accounts as accounts_service,
    error::KernelError,
    utils,
};


#[derive(Clone, Debug)]
pub struct RequestPasswordReset {
    pub metadata: EventMetadata,
}


#[derive(Clone, Debug)]
pub struct RequestPasswordResetNonStored {
    pub plaintext_token: String,
}


impl<'a> eventsourcing::Command<'a> for RequestPasswordReset {
    type Aggregate = accounts::Account;
    type Event = accounts::Event;
    type Context = PooledConnection<ConnectionManager<PgConnection>>;
    type Error = KernelError;
    type NonStoredData = RequestPasswordResetNonStored;

    fn validate(&self, _ctx: &Self::Context, _aggregate: &Self::Aggregate) -> Result<(), Self::Error> {
        return Ok(());
    }

    fn build_event(&self, _ctx: &Self::Context, aggregate: &Self::Aggregate) -> Result<(Self::Event, Self::NonStoredData), Self::Error> {
        let password_reset_id = uuid::Uuid::new_v4();
        let mut rng = rand::thread_rng();
        let token_length = rng.gen_range(accounts_service::PASSWORD_RESET_TOKEN_MIN_LENGTH, accounts_service::PASSWORD_RESET_TOKEN_MAX_LENGTH);
        let token = utils::random_hex_string(token_length as usize);
        let hashed_token = bcrypt::hash(&token, accounts_service::PASSWORD_RESET_TOKEN_BCRYPT_COST)
            .map_err(|_| KernelError::Bcrypt)?;

        let data = accounts::EventData::PasswordResetRequestedV1(accounts::PasswordResetRequestedV1{
            password_reset_id,
            password_reset_token: hashed_token.clone(),
        });

        return  Ok((accounts::Event{
            id: uuid::Uuid::new_v4(),
            timestamp: chrono::Utc::now(),
            data,
            aggregate_id: aggregate.id,
            metadata: self.metadata.clone(),
        }, RequestPasswordResetNonStored{plaintext_token: token}));
    }
}
