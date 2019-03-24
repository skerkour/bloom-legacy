use crate::{
    services::account::domain::account as account_domain,
    services::common::events::EventMetadata,
    services::account::validators,
    services::account,
    error::KernelError,
};
use diesel::{
    PgConnection,
    r2d2::{PooledConnection, ConnectionManager},
};


#[derive(Clone, Debug)]
pub struct UpdatePassword {
    pub current_password: String,
    pub new_password: String,
    pub metdata: EventMetadata,
}

impl<'a> eventsourcing::Command<'a> for UpdatePassword {
    type Aggregate = account_domain::Account;
    type Event = account_domain::Event;
    type Context = PooledConnection<ConnectionManager<PgConnection>>;
    type Error = KernelError;
    type NonStoredData = ();

    fn validate(&self, ctx: &Self::Context, aggregate: &Self::Aggregate) -> Result<(), Self::Error> {
        validators::password(&self.new_password)?;

        if aggregate.email == self.new_password {
            return Err(KernelError::Validation("Password cannot be your email address".to_string()));
        }

        if aggregate.username == self.new_password {
            return Err(KernelError::Validation("Password cannot be your username".to_string()));
        }

        if !bcrypt::verify(&self.current_password, &aggregate.password).map_err(|_| KernelError::Bcrypt)? {
            return Err(KernelError::Validation("Current password is not valid".to_string()));
        }

        return Ok(());
    }

    fn build_event(&self, _ctx: &Self::Context, aggregate: &Self::Aggregate) -> Result<(Self::Event, Self::NonStoredData), Self::Error> {
        let hashed_password = bcrypt::hash(&self.new_password, account::PASSWORD_BCRYPT_COST)
            .map_err(|_| KernelError::Bcrypt)?;

        let data = account_domain::EventData::PasswordUpdatedV1(account_domain::PasswordUpdatedV1{
            password: self.new_password.clone(),
        });

        return  Ok((account_domain::Event{
            id: uuid::Uuid::new_v4(),
            timestamp: chrono::Utc::now(),
            data,
            aggregate_id: aggregate.id,
            metadata: self.metdata.clone(),
        }, ()));
    }
}
