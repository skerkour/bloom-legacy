use crate::{error::KernelError, events::EventMetadata, myaccount::domain::account};
use diesel::{
    r2d2::{ConnectionManager, PooledConnection},
    PgConnection,
};

#[derive(Clone, Debug)]
pub struct UpdateEmail {
    pub email: String,
    pub metadata: EventMetadata,
}

impl eventsourcing::Command for UpdateEmail {
    type Aggregate = account::Account;
    type Event = account::Event;
    type Context = PooledConnection<ConnectionManager<PgConnection>>;
    type Error = KernelError;

    fn validate(
        &self,
        _ctx: &Self::Context,
        _aggregate: &Self::Aggregate,
    ) -> Result<(), Self::Error> {
        // validators::email(self.config.disposable_email_domains(), &self.email)?;

        // verify that an email isn't already in use
        // already done in pending emial verify

        return Ok(());
    }

    fn build_event(
        &self,
        _ctx: &Self::Context,
        aggregate: &Self::Aggregate,
    ) -> Result<Self::Event, Self::Error> {
        let data = account::EventData::EmailUpdatedV1(account::EmailUpdatedV1 {
            email: self.email.clone(),
        });

        return Ok(
            account::Event {
                id: uuid::Uuid::new_v4(),
                timestamp: chrono::Utc::now(),
                data,
                aggregate_id: aggregate.id,
                metadata: self.metadata.clone(),
            });
    }
}
