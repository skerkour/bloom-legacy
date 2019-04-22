use diesel::{
    PgConnection,
    r2d2::{PooledConnection, ConnectionManager},
};
use kernel::{
    KernelError,
    events::EventMetadata,
};
use crate::{
    domain::playlist,
    validators,
};


#[derive(Clone, Debug)]
pub struct Create {
    pub name: String,
    pub owner_id: uuid::Uuid,
    pub metadata: EventMetadata,
}

impl eventsourcing::Command for Create {
    type Aggregate = playlist::Playlist;
    type Event = playlist::Event;
    type Context = PooledConnection<ConnectionManager<PgConnection>>;
    type Error = KernelError;
    type NonStoredData = ();

    fn validate(&self, _ctx: &Self::Context, _aggregate: &Self::Aggregate) -> Result<(), Self::Error> {
        validators::playlist_name(&self.name)?;

        return Ok(());
    }

    fn build_event(&self, _ctx: &Self::Context, _aggregate: &Self::Aggregate) -> Result<(Self::Event, Self::NonStoredData), Self::Error> {
        let id = uuid::Uuid::new_v4();
        let data = playlist::EventData::CreatedV1(playlist::CreatedV1{
            id,
            name: self.name.clone(),
            owner_id: self.owner_id,
        });

        return  Ok((playlist::Event{
            id: uuid::Uuid::new_v4(),
            timestamp: chrono::Utc::now(),
            data,
            aggregate_id: id,
            metadata: self.metadata.clone(),
        }, ()));
    }
}
