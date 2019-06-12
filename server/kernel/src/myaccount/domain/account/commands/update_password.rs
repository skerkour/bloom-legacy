use crate::{
    myaccount::domain::account,
    events::EventMetadata,
    myaccount::validators,
    myaccount,
    error::KernelError,
    config::Config,
};
use diesel::{
    PgConnection,
    r2d2::{PooledConnection, ConnectionManager},
};


#[derive(Clone, Debug)]
pub struct UpdatePassword {
    pub current_password: String,
    pub new_password: String,
    pub config: Config,
    pub metadata: EventMetadata,
}

impl eventsourcing::Command for UpdatePassword {
    type Aggregate = account::Account;
    type Event = account::Event;
    type Context = PooledConnection<ConnectionManager<PgConnection>>;
    type Error = KernelError;
    type NonStoredData = ();

    fn validate(&self, _ctx: &Self::Context, aggregate: &Self::Aggregate) -> Result<(), Self::Error> {
        validators::password(self.config.basic_passwords(), &self.new_password)?;

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
        let hashed_password = bcrypt::hash(&self.new_password, myaccount::PASSWORD_BCRYPT_COST)
            .map_err(|_| KernelError::Bcrypt)?;

        let data = account::EventData::PasswordUpdatedV1(account::PasswordUpdatedV1{
            password: hashed_password,
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
