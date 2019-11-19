use crate::{config::Config, error::KernelError, accounts, accounts::domain::account};
use chrono::Utc;
use diesel::{
    r2d2::{ConnectionManager, PooledConnection},
    PgConnection,
};
use eventsourcing::{Event, EventTs};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug)]
pub struct ResetPassword {
    pub new_password: String,
    pub token: String,
    pub config: Config,
}

impl eventsourcing::Command for ResetPassword {
    type Aggregate = account::Account;
    type Event = PasswordReseted;
    type Context = PooledConnection<ConnectionManager<PgConnection>>;
    type Error = KernelError;

    fn validate(
        &self,
        _ctx: &Self::Context,
        aggregate: &Self::Aggregate,
    ) -> Result<(), Self::Error> {
        account::validators::password(self.config.basic_passwords.clone(), &self.new_password)?;
        let timestamp = Utc::now();
        let duration = aggregate.updated_at.signed_duration_since(timestamp);

        if aggregate.email == self.new_password {
            return Err(KernelError::Validation(
                "Password cannot be your email address".to_string(),
            ));
        }
        if aggregate.username == self.new_password {
            return Err(KernelError::Validation(
                "Password cannot be your username".to_string(),
            ));
        }

        if duration.num_minutes() > 30 {
            return Err(KernelError::Validation(
                "Code has expired, please reset your password again".to_string(),
            ));
        }

        // we can unwrap because if we are here it means that we found the account with it's password_reset_id
        if !argon2id::verify_password(aggregate.password_reset_token.unwrap().into(), self.token.as_bytes()) {
            return Err(KernelError::Validation("Token is not valid".to_string()));
        }

        return Ok(());
    }

    fn build_event(
        &self,
        _ctx: &Self::Context,
        _aggregate: &Self::Aggregate,
    ) -> Result<Self::Event, Self::Error> {
        let hashed_password = argon2id::hash_password(
            self.new_password.as_bytes(),
            accounts::PASSWORD_ARGON2_OPSLIMIT,
            accounts::PASSWORD_ARGON2_MEMLIMIT,
        )?.to_string();

        return Ok(PasswordReseted {
            timestamp: chrono::Utc::now(),
            password: hashed_password,
        });
    }
}

// Event
#[derive(Clone, Debug, Deserialize, EventTs, Serialize)]
pub struct PasswordReseted {
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub password: String, // hashed password
}

impl Event for PasswordReseted {
    type Aggregate = account::Account;

    fn apply(&self, aggregate: Self::Aggregate) -> Self::Aggregate {
        return Self::Aggregate {
            password: self.password.clone(),
            password_reset_id: None,
            password_reset_token: None,
            ..aggregate
        };
    }
}
