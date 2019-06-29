use crate::{
    error::KernelError, events::EventMetadata, myaccount::domain::account, myaccount::validators,
};
use diesel::{
    r2d2::{ConnectionManager, PooledConnection},
    PgConnection,
};

#[derive(Clone, Debug)]
pub struct UpdateBio {
    pub bio: String,
    pub metadata: EventMetadata,
}

impl eventsourcing::Command for UpdateBio {
    type Aggregate = account::Account;
    type Event = account::Event;
    type Context = PooledConnection<ConnectionManager<PgConnection>>;
    type Error = KernelError;
    type NonStoredData = ();

    fn validate(
        &self,
        _ctx: &Self::Context,
        _aggregate: &Self::Aggregate,
    ) -> Result<(), Self::Error> {
        validators::bio(&self.bio)?;

        return Ok(());
    }

    fn build_event(
        &self,
        _ctx: &Self::Context,
        aggregate: &Self::Aggregate,
    ) -> Result<(Self::Event, Self::NonStoredData), Self::Error> {
        let data = account::EventData::BioUpdatedV1(account::BioUpdatedV1 {
            bio: self.bio.clone(),
        });

        return Ok((
            account::Event {
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
