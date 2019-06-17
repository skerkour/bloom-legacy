use diesel::{
    PgConnection,
    r2d2::{PooledConnection, ConnectionManager},
};
use kernel::{
    KernelError,
    events::EventMetadata,
};
use crate::{
    domain::file,
    validators,
};


#[derive(Clone, Debug)]
pub struct Create {
    pub name: String,
    pub type_: String,
    pub size: i64,
    pub parent_id: Option<uuid::Uuid>,
    pub owner_id: uuid::Uuid,
    pub metadata: EventMetadata,
}

impl eventsourcing::Command for Create {
    type Aggregate = file::File;
    type Event = file::Event;
    type Context = PooledConnection<ConnectionManager<PgConnection>>;
    type Error = KernelError;
    type NonStoredData = ();

    // Validate to implement the goes.Command interface
    // TODO: verify size and type
    // TODO: checks in transaction
    // check parent exists
    // check name is valid
    // check name is not already taken by another parent's children
    // check that ownerID == parent.OwnerID
    fn validate(&self, _ctx: &Self::Context, _aggregate: &Self::Aggregate) -> Result<(), Self::Error> {
        validators::file_name(&self.name)?;
        return Ok(());
    }

    fn build_event(&self, _ctx: &Self::Context, _aggregate: &Self::Aggregate) -> Result<(Self::Event, Self::NonStoredData), Self::Error> {
        let id = uuid::Uuid::new_v4();

        let event_data = file::EventData::CreatedV1(file::CreatedV1{
            id,
            parent_id: self.parent_id,
            name: self.name.clone(),
            size: self.size,
            type_: self.type_.clone(), // MIME type
            owner_id: self.owner_id,
        });

        return  Ok((file::Event{
            id: uuid::Uuid::new_v4(),
            timestamp: chrono::Utc::now(),
            data: event_data,
            aggregate_id: id,
            metadata: self.metadata.clone(),
        }, ()));
    }
}
