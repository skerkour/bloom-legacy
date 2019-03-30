use crate::{
    services::account::domain::account,
    services::common::events::EventMetadata,
    services::account::validators,
    error::KernelError,
};
use diesel::{
    PgConnection,
    r2d2::{PooledConnection, ConnectionManager},
};


#[derive(Clone, Debug)]
pub struct UpdateEmail {
    pub email: String,
    pub metadata: EventMetadata,
}

impl<'a> eventsourcing::Command<'a> for UpdateEmail {
    type Aggregate = account::Account;
    type Event = account::Event;
    type Context = PooledConnection<ConnectionManager<PgConnection>>;
    type Error = KernelError;
    type NonStoredData = ();

    fn validate(&self, ctx: &Self::Context, _aggregate: &Self::Aggregate) -> Result<(), Self::Error> {
        use crate::db::schema::{
            account_accounts::dsl::*,
        };
        use diesel::prelude::*;

        validators::email(&self.email)?;

        // verify that an email isn't already in use
        // already done in pending emial verify

        // let existing_email: i64 = account_accounts
        //     .filter(email.eq(&self.email))
        //     .filter(deleted_at.is_null())
        //     .count()
        //     .get_result(ctx)?;
        // if existing_email != 0 {
        //     return Err(KernelError::Validation(format!("Email: {} is already in use.", &self.email)));
        // }

        return Ok(());
    }

    fn build_event(&self, _ctx: &Self::Context, aggregate: &Self::Aggregate) -> Result<(Self::Event, Self::NonStoredData), Self::Error> {
        let data = account::EventData::EmailUpdatedV1(account::EmailUpdatedV1{
            email: self.email.clone(),
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
