use crate::{
    myaccount,
    myaccount::domain::account,
    events::EventMetadata,
    error::KernelError,
    utils,
};
use diesel::{
    PgConnection,
    r2d2::{PooledConnection, ConnectionManager},
};


#[derive(Clone, Debug)]
pub struct Delete {
    pub metadata: EventMetadata,
}

impl eventsourcing::Command for Delete {
    type Aggregate = account::Account;
    type Event = account::Event;
    type Context = PooledConnection<ConnectionManager<PgConnection>>;
    type Error = KernelError;
    type NonStoredData = ();

    fn validate(&self, _ctx: &Self::Context, aggregate: &Self::Aggregate) -> Result<(), Self::Error> {
        if aggregate.deleted_at.is_some() {
            return Err(KernelError::Validation("Account is already deleted".to_string()));
        }

        Ok(())
    }

    fn build_event(&self, _ctx: &Self::Context, aggregate: &Self::Aggregate) -> Result<(Self::Event, Self::NonStoredData), Self::Error> {
        let random_string = utils::random_hex_string(10);
        let random_password = utils::random_hex_string(100);
        let password = bcrypt::hash(random_password, myaccount::PASSWORD_BCRYPT_COST)
            .map_err(|_| KernelError::Bcrypt)?;

        let event_data = account::EventData::DeletedV1(account::DeletedV1{
            random_string,
            password,
        });

        return  Ok((account::Event{
            id: uuid::Uuid::new_v4(),
            timestamp: chrono::Utc::now(),
            data: event_data,
            aggregate_id: aggregate.id,
            metadata: self.metadata.clone(),
        }, ()));
    }
}
