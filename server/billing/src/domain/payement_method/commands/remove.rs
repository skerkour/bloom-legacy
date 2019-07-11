use diesel::{
    PgConnection,
    r2d2::{PooledConnection, ConnectionManager},
};
use kernel::{
    KernelError,
    events::EventMetadata,
};
use crate::{
    domain::payment_method,
};


#[derive(Clone, Debug)]
pub struct Remove {
    pub metadata: EventMetadata,
}

impl eventsourcing::Command for Remove {
    type Aggregate = payment_method::PaymentMethod;
    type Event = payment_method::Event;
    type Context = PooledConnection<ConnectionManager<PgConnection>>;
    type Error = KernelError;

    fn validate(&self, _ctx: &Self::Context, _aggregate: &Self::Aggregate) -> Result<(), Self::Error> {
        return Ok(());
    }

    fn build_event(&self, _ctx: &Self::Context, _aggregate: &Self::Aggregate) -> Result<(Self::Event, Self::NonStoredData), Self::Error> {
        let data = payment_method::EventData::RemovedV1;

        return  Ok((payment_method::Event{
            id: uuid::Uuid::new_v4(),
            timestamp: chrono::Utc::now(),
            data,
            aggregate_id: id,
            metadata: self.metadata.clone(),
        }, ()));
    }
}
