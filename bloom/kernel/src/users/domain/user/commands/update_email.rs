use crate::{
    users::domain::user,
    events::EventMetadata,
    users::validators,
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
    type Aggregate = user::User;
    type Event = user::Event;
    type Context = PooledConnection<ConnectionManager<PgConnection>>;
    type Error = KernelError;
    type NonStoredData = ();

    fn validate(&self, ctx: &Self::Context, _aggregate: &Self::Aggregate) -> Result<(), Self::Error> {
        use crate::db::schema::{
            kernel_users::dsl::*,
        };
        use diesel::prelude::*;

        validators::email(&self.email)?;

        // verify that an email isn't already in use
        // already done in pending emial verify

        // let existing_email: i64 = kernel_users
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
        let data = user::EventData::EmailUpdatedV1(user::EmailUpdatedV1{
            email: self.email.clone(),
        });

        return  Ok((user::Event{
            id: uuid::Uuid::new_v4(),
            timestamp: chrono::Utc::now(),
            data,
            aggregate_id: aggregate.id,
            metadata: self.metadata.clone(),
        }, ()));
    }
}
