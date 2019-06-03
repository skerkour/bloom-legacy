use diesel::{
    PgConnection,
    r2d2::{PooledConnection, ConnectionManager},
};
use kernel::{
    KernelError,
    events::EventMetadata,
};
use crate::{
    domain::profile,
};


#[derive(Clone, Debug)]
pub struct UpdateUsedSpace {
    pub space: i64,
    pub metadata: EventMetadata,
}

impl eventsourcing::Command for UpdateUsedSpace {
    type Aggregate = profile::Profile;
    type Event = profile::Event;
    type Context = PooledConnection<ConnectionManager<PgConnection>>;
    type Error = KernelError;
    type NonStoredData = ();

    fn validate(&self, _ctx: &Self::Context, _aggregate: &Self::Aggregate) -> Result<(), Self::Error> {
        return Ok(());
    }

    fn build_event(&self, _ctx: &Self::Context, aggregate: &Self::Aggregate) -> Result<(Self::Event, Self::NonStoredData), Self::Error> {

        let data = profile::EventData::UsedSpaceUpdatedV1(profile::UsedSpaceUpdatedV1{
            space: self.space,
        });

        return  Ok((profile::Event{
            id: uuid::Uuid::new_v4(),
            timestamp: chrono::Utc::now(),
            data,
            aggregate_id: aggregate.id,
            metadata: self.metadata.clone(),
        }, ()));
    }
}
