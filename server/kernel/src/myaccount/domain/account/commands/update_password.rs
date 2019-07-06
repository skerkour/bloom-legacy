use crate::{
    config::Config, error::KernelError, events::EventMetadata, myaccount,
    myaccount::domain::account, myaccount::validators,
};
use diesel::{
    r2d2::{ConnectionManager, PooledConnection},
    PgConnection,
};
use eventsourcing::{Event, EventTs};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug)]
pub struct UpdatePassword {
    pub current_password: String,
    pub new_password: String,
    pub config: Config,
}

impl eventsourcing::Command for UpdatePassword {
    type Aggregate = account::Account;
    type Event = PasswordUpdated;
    type Context = PooledConnection<ConnectionManager<PgConnection>>;
    type Error = KernelError;

    fn validate(
        &self,
        _ctx: &Self::Context,
        aggregate: &Self::Aggregate,
    ) -> Result<(), Self::Error> {
        validators::password(self.config.basic_passwords.clone(), &self.new_password)?;

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

        if !bcrypt::verify(&self.current_password, &aggregate.password)
            .map_err(|_| KernelError::Bcrypt)?
        {
            return Err(KernelError::Validation(
                "Current password is not valid".to_string(),
            ));
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

        return Ok(PasswordUpdated {
            timestamp: chrono::Utc::now(),
            password: hashed_password,
        });
    }
}


// Event
#[derive(Clone, Debug, Deserialize, Eventts, Serialize)]
pub struct PasswordUpdated {
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub password: String,
}

impl Event for PasswordUpdated {
    type Aggregate = super::Account;

    fn apply(&self, aggregate: Self::Aggregate) -> Self::Aggregate {
        return Self::Aggregate {
            password: self.password.clone(),
            ..aggregate
        };
    }
}
