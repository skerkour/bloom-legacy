use crate::domain::profile;
use diesel::{
    r2d2::{ConnectionManager, PooledConnection},
    PgConnection,
};
use kernel::{events::EventMetadata, KernelError};

#[derive(Clone, Debug)]
pub struct Create {
    pub account_id: uuid::Uuid,
    pub metadata: EventMetadata,
}

impl eventsourcing::Command for Create {
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
        _aggregate: &Self::Aggregate,
    ) -> Result<(Self::Event, Self::NonStoredData), Self::Error> {
        let id = uuid::Uuid::new_v4();

        let data = profile::EventData::CreatedV1(profile::CreatedV1 {
            id,
            account_id: self.account_id,
        });

        return Ok((
            profile::Event {
                id: uuid::Uuid::new_v4(),
                timestamp: chrono::Utc::now(),
                data,
                aggregate_id: id,
                metadata: self.metadata.clone(),
            },
            (),
        ));
    }
}
