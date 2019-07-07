use crate::{
    error::KernelError, myaccount, myaccount::domain::account,
    myaccount::validators,
};
use diesel::{
    r2d2::{ConnectionManager, PooledConnection},
    PgConnection,
};
use eventsourcing::{Event, EventTs};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug)]
pub struct Create {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
    pub username: String,
    pub host: String,
}

impl eventsourcing::Command for Create {
    type Aggregate = account::Account;
    type Event = account::Created;
    type Context = PooledConnection<ConnectionManager<PgConnection>>;
    type Error = KernelError;

    fn validate(
        &self,
        ctx: &Self::Context,
        _aggregate: &Self::Aggregate,
    ) -> Result<(), Self::Error> {
        use crate::db::schema::kernel_accounts::dsl::*;
        use diesel::prelude::*;

        validators::username(&self.username)?;

        // verify that an email isn't already in use
        let existing_email: i64 = kernel_accounts
            .filter(email.eq(&self.email))
            .filter(deleted_at.is_null())
            .count()
            .get_result(ctx)?;
        if existing_email != 0 {
            return Err(KernelError::Validation(format!(
                "Email: {} is already in use.",
                &self.email
            )));
        }

        // verify that username isn't already in use
        let existing_username: i64 = kernel_accounts
            .filter(username.eq(&self.username))
            .count()
            .get_result(ctx)?;
        if existing_username != 0 {
            return Err(KernelError::Validation(format!(
                "Username: {} is already in use.",
                &self.username
            )));
        }

        return Ok(());
    }

    fn build_event(
        &self,
        _ctx: &Self::Context,
        _aggregate: &Self::Aggregate,
    ) -> Result<Self::Event, Self::Error> {
        return Ok(account::Created {
            timestamp: chrono::Utc::now(),
            id: uuid::Uuid::new_v4(),
            first_name: self.first_name.clone(),
            last_name: self.last_name.clone(),
            email: self.email.clone(),
            password: self.password.clone(),
            avatar_url: myaccount::AVATAR_DEFAULT_PATH.to_string(),
            username: self.username.clone(),
            is_admin: false,
        });
    }
}

// Event
#[derive(Clone, Debug, Deserialize, EventTs, Serialize)]
pub struct Created {
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub id: uuid::Uuid,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
    pub avatar_url: String,
    pub username: String,
    pub is_admin: bool,
}

impl Event for Created {
    type Aggregate = account::Account;

    fn apply(&self, aggregate: Self::Aggregate) -> Self::Aggregate {
        return Self::Aggregate {
            id: self.id,
            created_at: self.timestamp,
            updated_at: self.timestamp,
            deleted_at: None,
            version: 0,
            avatar_url: self.avatar_url.clone(),
            email: self.email.clone(),
            first_name: self.first_name.clone(),
            is_admin: self.is_admin,
            last_name: self.last_name.clone(),
            password: self.password.clone(),
            password_reset_id: None,
            password_reset_token: None,
            username: self.username.clone(),
            disabled_at: None,
            bio: String::new(),
            display_name: self.username.clone(),
        };
    }
}
