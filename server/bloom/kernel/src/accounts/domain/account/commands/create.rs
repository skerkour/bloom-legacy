use crate::accounts::domain::account;
use bloom_error::BloomError;
use diesel::{
    r2d2::{ConnectionManager, PooledConnection},
    PgConnection,
};
use eventsourcing::{Event, EventTs};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug)]
pub struct Create {
    pub display_name: String,
    pub email: String,
    pub auth_key_hash: String,
    pub username: String,
}

impl eventsourcing::Command for Create {
    type Aggregate = account::Account;
    type Event = account::Created;
    type Context = PooledConnection<ConnectionManager<PgConnection>>;
    type Error = BloomError;

    fn validate(
        &self,
        ctx: &Self::Context,
        _aggregate: &Self::Aggregate,
    ) -> Result<(), Self::Error> {
        use crate::db::schema::{accounts, deleted_usernames};
        use diesel::prelude::*;

        bloom_validators::accounts::username(&self.username)?;

        // verify that an email isn't already in use
        let existing_email: i64 = accounts::dsl::accounts
            .filter(accounts::dsl::email.eq(&self.email))
            .count()
            .get_result(ctx)?;
        if existing_email != 0 {
            return Err(BloomError::Validation(format!(
                "Email: {} is already in use.",
                &self.email
            )));
        }

        // verify that username isn't already in use
        let existing_username: i64 = accounts::dsl::accounts
            .filter(accounts::dsl::username.eq(&self.username))
            .count()
            .get_result(ctx)?;
        if existing_username != 0 {
            return Err(BloomError::Validation(format!(
                "Username: {} is already in use.",
                &self.username
            )));
        }

        // verify that username was not used by a deleted account
        let existing_deleted_username: i64 =
            deleted_usernames::dsl::deleted_usernames
                .filter(deleted_usernames::dsl::username.eq(&self.username))
                .count()
                .get_result(ctx)?;
        if existing_deleted_username != 0 {
            return Err(BloomError::Validation(format!(
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
            display_name: self.display_name.clone(),
            email: self.email.clone(),
            auth_key_hash: self.auth_key_hash.clone(),
            username: self.username.clone(),
        });
    }
}

// Event
#[derive(Clone, Debug, Deserialize, EventTs, Serialize)]
pub struct Created {
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub id: uuid::Uuid,
    pub display_name: String,
    pub email: String,
    pub auth_key_hash: String,
    pub username: String,
}

impl Event for Created {
    type Aggregate = account::Account;

    fn apply(&self, _aggregate: Self::Aggregate) -> Self::Aggregate {
        return Self::Aggregate {
            id: self.id,
            created_at: self.timestamp,
            updated_at: self.timestamp,
            avatar_id: None,
            email: self.email.clone(),
            is_admin: false,
            last_name: String::new(),
            first_name: String::new(),
            auth_key_hash: self.auth_key_hash.clone(),
            username: self.username.clone(),
            disabled_at: None,
            bio: String::new(),
            display_name: self.display_name.clone(),
        };
    }
}
