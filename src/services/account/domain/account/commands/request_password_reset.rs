use crate::{
    services::account::domain::account as account_domain,
    services::common::events::EventMetadata,
    services::account::validators,
    services::account,
    error::KernelError,
    services::common::utils,
};
use diesel::{
    PgConnection,
    r2d2::{PooledConnection, ConnectionManager},
};
use rand::Rng;


#[derive(Clone, Debug)]
pub struct RequestPasswordReset {
    pub metadata: EventMetadata,
}


#[derive(Clone, Debug)]
pub struct RequestPasswordResetNonStored {
    pub cleartext_token: String,
}


impl<'a> eventsourcing::Command<'a> for RequestPasswordReset {
    type Aggregate = account_domain::Account;
    type Event = account_domain::Event;
    type Context = PooledConnection<ConnectionManager<PgConnection>>;
    type Error = KernelError;
    type NonStoredData = RequestPasswordResetNonStored;

    fn validate(&self, ctx: &Self::Context, aggregate: &Self::Aggregate) -> Result<(), Self::Error> {
        return Ok(());
    }

    fn build_event(&self, _ctx: &Self::Context, aggregate: &Self::Aggregate) -> Result<(Self::Event, Self::NonStoredData), Self::Error> {
        let password_reset_id = uuid::Uuid::new_v4();
        let mut rng = rand::thread_rng();
        let token_length = rng.gen_range(account::PASSWORD_RESET_TOKEN_MIN_LENGTH, account::PASSWORD_RESET_TOKEN_MAX_LENGTH);
        let token = utils::random_hex_string(token_length as usize);
        let hashed_token = bcrypt::hash(&token, account::PASSWORD_RESET_TOKEN_BCRYPT_COST)
            .map_err(|_| KernelError::Bcrypt)?;

        let data = account_domain::EventData::PasswordResetRequestedV1(account_domain::PasswordResetRequestedV1{
            password_reset_id,
            password_reset_token: hashed_token.clone(),
        });

        return  Ok((account_domain::Event{
            id: uuid::Uuid::new_v4(),
            timestamp: chrono::Utc::now(),
            data,
            aggregate_id: aggregate.id,
            metadata: self.metadata.clone(),
        }, RequestPasswordResetNonStored{cleartext_token: token}));
    }
}
