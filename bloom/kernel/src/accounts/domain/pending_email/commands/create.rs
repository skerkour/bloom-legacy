use crate::{
    accounts::validators,
    error::KernelError,
    accounts::domain::pending_email,
    accounts,
    events::EventMetadata,
    utils,
};
use diesel::{
    PgConnection,
    r2d2::{PooledConnection, ConnectionManager},
};


#[derive(Clone, Debug)]
pub struct Create {
    pub email: String,
    pub account_id: uuid::Uuid,
    pub metadata: EventMetadata,
}

#[derive(Clone, Debug)]
pub struct CreateNonStored {
    pub code: String,
}

impl<'a> eventsourcing::Command<'a> for Create {
    type Aggregate = pending_email::PendingEmail;
    type Event = pending_email::Event;
    type Context = PooledConnection<ConnectionManager<PgConnection>>;
    type Error = KernelError;
    type NonStoredData = CreateNonStored;

    fn validate(&self, ctx: &Self::Context, _aggregate: &Self::Aggregate) -> Result<(), Self::Error> {
        use crate::db::schema::{
            kernel_accounts::dsl::*,
        };
        use diesel::prelude::*;

        validators::email(&self.email)?;

        // verify that an email isn't already in use
        let existing_email: i64 = kernel_accounts
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
        let new_pending_email_id = uuid::Uuid::new_v4();
        let code = utils::random_digit_string(8);
        let token = bcrypt::hash(&code, accounts::PENDING_EMAIL_TOKEN_BCRYPT_COST)
            .map_err(|_| KernelError::Bcrypt)?;

        let data = pending_email::EventData::CreatedV1(pending_email::CreatedV1{
            id: new_pending_email_id,
            email: self.email.clone(),
            account_id: self.account_id,
            token,
        });

        let non_stored = CreateNonStored{
            code,
        };

        return  Ok((pending_email::Event{
            id: uuid::Uuid::new_v4(),
            timestamp: now,
            data,
            aggregate_id: new_pending_email_id,
            metadata: self.metadata.clone(),
        }, non_stored));
    }
}
