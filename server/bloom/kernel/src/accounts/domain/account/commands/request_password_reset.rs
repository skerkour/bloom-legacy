use crate::{error::KernelError, accounts, accounts::domain::account, utils};
use diesel::{
    r2d2::{ConnectionManager, PooledConnection},
    PgConnection,
};
use eventsourcing::{Event, EventTs};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug)]
pub struct RequestPasswordReset {}

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
        _aggregate: &Self::Aggregate,
    ) -> Result<Self::Event, Self::Error> {
        let password_reset_id = uuid::Uuid::new_v4();
        let token = utils::random_base64_string(accounts::PASSWORD_RESET_TOKEN_BYTES as usize);
        let hashed_token = argon2id::hash_password(
            token.as_bytes(),
            accounts::PASSWORD_RESET_TOKEN_ARGON2_OPSLIMIT,
            accounts::PASSWORD_RESET_TOKEN_ARGON2_MEMLIMIT,
        )?.to_string();

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
