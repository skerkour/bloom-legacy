use crate::{
    error::KernelError, myaccount, myaccount::domain::account, utils,
};
use diesel::{
    r2d2::{ConnectionManager, PooledConnection},
    PgConnection,
};
use eventsourcing::{Event, EventTs};
use rand::Rng;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug)]
pub struct RequestPasswordReset {}

#[derive(Clone, Debug)]
pub struct RequestPasswordResetNonStored {
    pub plaintext_token: String,
}

impl eventsourcing::Command for RequestPasswordReset {
    type Aggregate = account::Account;
    type Event = PasswordResetRequested;
    type Context = PooledConnection<ConnectionManager<PgConnection>>;
    type Error = KernelError;

    fn validate(
        &self,
        _ctx: &Self::Context,
        _aggregate: &Self::Aggregate,
    ) -> Result<(), Self::Error> {
        return Ok(());
    }

    fn build_event(
        &self,
        _ctx: &Self::Context,
        aggregate: &Self::Aggregate,
    ) -> Result<Self::Event, Self::Error> {
        let password_reset_id = uuid::Uuid::new_v4();
        let mut rng = rand::thread_rng();
        let token_length = rng.gen_range(
            myaccount::PASSWORD_RESET_TOKEN_MIN_LENGTH,
            myaccount::PASSWORD_RESET_TOKEN_MAX_LENGTH,
        );
        let token = utils::random_hex_string(token_length as usize);
        let hashed_token = bcrypt::hash(&token, myaccount::PASSWORD_RESET_TOKEN_BCRYPT_COST)
            .map_err(|_| KernelError::Bcrypt)?;

        return Ok(PasswordResetRequested {
            timestamp: chrono::Utc::now(),
            password_reset_id,
            password_reset_token: hashed_token.clone(),
            token_plaintext: token,
        });
    }
}

// Event
#[derive(Clone, Debug, Deserialize, EventTs, Serialize)]
pub struct PasswordResetRequested {
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub password_reset_id: uuid::Uuid,
    pub password_reset_token: String, // hashed token
    pub token_plaintext: String,
}

impl Event for PasswordResetRequested {
    type Aggregate = account::Account;

    fn apply(&self, aggregate: Self::Aggregate) -> Self::Aggregate {
        return Self::Aggregate {
            password_reset_id: Some(self.password_reset_id),
            password_reset_token: Some(self.password_reset_token.clone()),
            ..aggregate
        };
    }
}
