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
pub struct Add {
    pub billing_profile_id: uuid::Uuid,
    pub details: PaymentDetails,
    pub is_default: bool,
    pub metadata: EventMetadata,
}

impl eventsourcing::Command for Add {
    type Aggregate = payment_method::PaymentMethod;
    type Event = payment_method::Event;
    type Context = PooledConnection<ConnectionManager<PgConnection>>;
    type Error = KernelError;

    fn validate(&self, _ctx: &Self::Context, _aggregate: &Self::Aggregate) -> Result<(), Self::Error> {
        return Ok(());
    }

    fn build_event(&self, _ctx: &Self::Context, _aggregate: &Self::Aggregate) -> Result<(Self::Event, Self::NonStoredData), Self::Error> {
        let id = uuid::Uuid::new_v4();

        let data = payment_method::EventData::AddedV1(payment_method::AddedV1{
            id: id,
            details: self.details.clone(),
            is_default: self.is_default,
            billing_profile_id: self.account_id,
        });

        return  Ok((payment_method::Event{
            id: uuid::Uuid::new_v4(),
            timestamp: chrono::Utc::now(),
            data,
            aggregate_id: id,
            metadata: self.metadata.clone(),
        }, ()));
    }
}
