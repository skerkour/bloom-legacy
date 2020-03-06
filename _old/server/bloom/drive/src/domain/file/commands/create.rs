use crate::domain::file;
use diesel::{
    r2d2::{ConnectionManager, PooledConnection},
    PgConnection,
};
use eventsourcing::{Event, EventTs};
use kernel::KernelError;

#[derive(Clone, Debug)]
pub struct Create {
    pub name: String,
    pub type_: String,
    pub size: i64,
    pub parent_id: Option<uuid::Uuid>,
    pub owner_id: uuid::Uuid,
}

impl eventsourcing::Command for Create {
    type Aggregate = file::File;
    type Event = Created;
    type Context = PooledConnection<ConnectionManager<PgConnection>>;
    type Error = KernelError;

    // Validate to implement the goes.Command interface
    // TODO: verify size and type
    // TODO: checks in transaction
    // check parent exists
    // check name is valid
    // check name is not already taken by another parent's children
    // check that ownerID == parent.OwnerID
    fn validate(
        &self,
        _ctx: &Self::Context,
        _aggregate: &Self::Aggregate,
    ) -> Result<(), Self::Error> {
        file::validators::name(&self.name)?;
        return Ok(());
    }

    fn build_event(
        &self,
        _ctx: &Self::Context,
        _aggregate: &Self::Aggregate,
    ) -> Result<Self::Event, Self::Error> {
        return Ok(Created {
            id: uuid::Uuid::new_v4(),
            timestamp: chrono::Utc::now(),
            parent_id: self.parent_id,
            name: self.name.clone(),
            size: self.size,
            type_: self.type_.clone(), // MIME type
            owner_id: self.owner_id,
        });
    }
}

// Event
#[derive(Clone, Debug, EventTs)]
pub struct Created {
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub id: uuid::Uuid,
    pub name: String,
    pub parent_id: Option<uuid::Uuid>,
    pub size: i64,
    pub type_: String, // MIME type
    pub owner_id: uuid::Uuid,
}

impl Event for Created {
    type Aggregate = file::File;

    fn apply(&self, _aggregate: Self::Aggregate) -> Self::Aggregate {
        return Self::Aggregate {
            id: self.id,
            created_at: self.timestamp,
            updated_at: self.timestamp,
            version: 0,
            explicitly_trashed: false,
            name: self.name.clone(),
            parent_id: self.parent_id,
            size: self.size,
            type_: self.type_.clone(),
            trashed_at: None,
            owner_id: self.owner_id,
        };
    }
}
