use crate::{
    users::validators,
    error::KernelError,
    users::domain::pending_user,
    user,
    events::EventMetadata,
    utils,
};
use diesel::{
    PgConnection,
    r2d2::{PooledConnection, ConnectionManager},
};


#[derive(Clone, Debug)]
pub struct Create {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
    pub metadata: EventMetadata,
}

#[derive(Clone, Debug)]
pub struct CreateNonStored {
    pub code: String,
}


impl<'a> eventsourcing::Command<'a> for Create {
    type Aggregate = pending_user::PendingUser;
    type Event = pending_user::Event;
    type Context = PooledConnection<ConnectionManager<PgConnection>>;
    type Error = KernelError;
    type NonStoredData = CreateNonStored;

    fn validate(&self, ctx: &Self::Context, _aggregate: &Self::Aggregate) -> Result<(), Self::Error> {
        use crate::db::schema::{
            kernel_users::dsl::*,
        };
        use diesel::prelude::*;


        validators::first_name(&self.first_name)?;
        validators::last_name(&self.last_name)?;
        validators::password(&self.password)?;
        validators::email(&self.email)?;

        if self.password == self.email {
            return Err(KernelError::Validation("password must be different than your email address".to_string()));
        }

        // verify that an email isn't already in use
        let existing_email: i64 = kernel_users
            .filter(email.eq(&self.email))
            .filter(deleted_at.is_null())
            .count()
            .get_result(ctx)?;
        if existing_email != 0 {
            return Err(KernelError::Validation(format!("Email: {} is already in use.", &self.email)));
        }

        return Ok(());
    }

    fn build_event(&self, _ctx: &Self::Context, _aggregate: &Self::Aggregate) -> Result<(Self::Event, Self::NonStoredData), Self::Error> {
        let now = chrono::Utc::now();
        let new_pending_user_id = uuid::Uuid::new_v4();
        let code = utils::random_digit_string(8);
        let hashed_password = bcrypt::hash(&self.password, user::PASSWORD_BCRYPT_COST)
            .map_err(|_| KernelError::Bcrypt)?;
        let token = bcrypt::hash(&code, user::PENDING_user_TOKEN_BCRYPT_COST)
            .map_err(|_| KernelError::Bcrypt)?;

        let data = pending_user::EventData::CreatedV1(pending_user::CreatedV1{
            id: new_pending_user_id,
            first_name: self.first_name.clone(),
            last_name: self.last_name.clone(),
            email: self.email.clone(),
            password: hashed_password,
            token,
        });

        let non_stored = CreateNonStored{
            code,
        };

        return  Ok((pending_user::Event{
            id: uuid::Uuid::new_v4(),
            timestamp: now,
            data,
            aggregate_id: new_pending_user_id,
            metadata: self.metadata.clone(),
        }, non_stored));
    }
}
