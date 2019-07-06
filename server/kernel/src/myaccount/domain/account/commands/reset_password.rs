use crate::{
    config::Config, error::KernelError, events::EventMetadata, myaccount,
    myaccount::domain::account, myaccount::validators,
};
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
        validators::password(self.config.basic_passwords.clone(), &self.new_password)?;
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
        if !bcrypt::verify(
            &self.token,
            aggregate.password_reset_token.as_ref().unwrap(),
        )? {
            return Err(KernelError::Validation("Token is not valid".to_string()));
        }

        return Ok(());
    }

    fn build_event(
        &self,
        _ctx: &Self::Context,
        aggregate: &Self::Aggregate,
    ) -> Result<Self::Event, Self::Error> {
        let hashed_password = bcrypt::hash(&self.new_password, myaccount::PASSWORD_BCRYPT_COST)
            .map_err(|_| KernelError::Bcrypt)?;

        return Ok(account::Event {
            timestamp: chrono::Utc::now(),
            password: hashed_password,,
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
    type Aggregate = super::Account;

    fn apply(&self, aggregate: Self::Aggregate) -> Self::Aggregate {
        return Self::Aggregate {
            password: self.password.clone(),
            password_reset_id: None,
            password_reset_token: None,
            ..aggregate
        };
    }
}
