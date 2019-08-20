use crate::domain::profile;
use diesel::{
    r2d2::{ConnectionManager, PooledConnection},
    PgConnection,
};
use kernel::{events::EventMetadata, KernelError};

#[derive(Clone, Debug)]
pub struct UpdateStripeCustomerId {
    pub stripe_customer_id: String,
    pub metadata: EventMetadata,
}

impl eventsourcing::Command for UpdateStripeCustomerId {
    type Aggregate = profile::Profile;
    type Event = profile::Event;
    type Context = PooledConnection<ConnectionManager<PgConnection>>;
    type Error = KernelError;

    fn validate(
        &self,
        _ctx: &Self::Context,
        _aggregate: &Self::Aggregate,
    ) -> Result<(), Self::Error> {
        return Ok(());
    }

    fn build_event(
        &self,
        _ctx: &Self::Context,
        aggregate: &Self::Aggregate,
    ) -> Result<(Self::Event, Self::NonStoredData), Self::Error> {
        let data =
            profile::EventData::StripeCustomerIdUpdatedV1(profile::StripeCustomerIdUpdatedV1 {
                stripe_customer_id: self.stripe_customer_id.clone(),
            });

        return Ok((
            profile::Event {
                id: uuid::Uuid::new_v4(),
                timestamp: chrono::Utc::now(),
                data,
                aggregate_id: aggregate.id,
                metadata: self.metadata.clone(),
            },
            (),
        ));
    }
}
