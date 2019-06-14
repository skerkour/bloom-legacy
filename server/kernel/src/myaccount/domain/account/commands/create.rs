use diesel::{
    PgConnection,
    r2d2::{PooledConnection, ConnectionManager},
};
use crate::{
    myaccount::domain::account,
    events::EventMetadata,
    myaccount::validators,
    error::KernelError,
    myaccount,
};

#[derive(Clone, Debug)]
pub struct Create {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
    pub username: String,
    pub host: String,
    pub metadata: EventMetadata,
}

impl eventsourcing::Command for Create {
    type Aggregate = account::Account;
    type Event = account::Event;
    type Context = PooledConnection<ConnectionManager<PgConnection>>;
    type Error = KernelError;
    type NonStoredData = ();

    fn validate(&self, ctx: &Self::Context, _aggregate: &Self::Aggregate) -> Result<(), Self::Error> {
        use crate::db::schema::{
            kernel_accounts::dsl::*,
        };
        use diesel::prelude::*;

        validators::username(&self.username)?;

        // verify that an email isn't already in use
        let existing_email: i64 = kernel_accounts
            .filter(email.eq(&self.email))
            .filter(deleted_at.is_null())
            .count()
            .get_result(ctx)?;
        if existing_email != 0 {
            return Err(KernelError::Validation(format!("Email: {} is already in use.", &self.email)));
        }

        // verify that username isn't already in use
        let existing_username: i64 = kernel_accounts
            .filter(username.eq(&self.username))
            .count()
            .get_result(ctx)?;
        if existing_username != 0 {
            return Err(KernelError::Validation(format!("Username: {} is already in use.", &self.username)));
        }

        return Ok(());
    }

    fn build_event(&self, _ctx: &Self::Context, _aggregate: &Self::Aggregate) -> Result<(Self::Event, Self::NonStoredData), Self::Error> {
        let now = chrono::Utc::now();
        let id = uuid::Uuid::new_v4();
        let mut metadata = self.metadata.clone();
        metadata.actor_id = Some(id);
        let data = account::EventData::CreatedV1(account::CreatedV1{
            id,
            first_name: self.first_name.clone(),
            last_name: self.last_name.clone(),
            email: self.email.clone(),
            password: self.password.clone(),
            avatar_url: format!("{}{}", &self.host, myaccount::AVATAR_DEFAULT_PATH),
            username: self.username.clone(),
            is_admin: false,
        });

        return  Ok((account::Event{
            id: uuid::Uuid::new_v4(),
            timestamp: now,
            data,
            aggregate_id: id,
            metadata,
        }, ()));
    }
}
