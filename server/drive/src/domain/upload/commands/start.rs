use crate::domain::{file, upload};
use diesel::{
    r2d2::{ConnectionManager, PooledConnection},
    PgConnection,
};
use eventsourcing::{Event, EventTs};
use kernel::KernelError;

#[derive(Clone, Debug)]
pub struct Start {
    pub file_name: String,
    pub parent_id: Option<uuid::Uuid>,
    pub owner_id: uuid::Uuid,
}

impl eventsourcing::Command for Start {
    type Aggregate = upload::Upload;
    type Event = Started;
    type Context = PooledConnection<ConnectionManager<PgConnection>>;
    type Error = KernelError;

    fn validate(
        &self,
        _ctx: &Self::Context,
        _aggregate: &Self::Aggregate,
    ) -> Result<(), Self::Error> {
        file::validators::name(&self.file_name)?;
        // TODO: check that parent exists

        return Ok(());
    }

    fn build_event(
        &self,
        _ctx: &Self::Context,
        _aggregate: &Self::Aggregate,
    ) -> Result<Self::Event, Self::Error> {
        return Ok(Started {
            id: uuid::Uuid::new_v4(),
            timestamp: chrono::Utc::now(),
            file_name: self.file_name.clone(),
            file_id: uuid::Uuid::new_v4(),
            parent_id: self.parent_id,
            owner_id: self.owner_id,
        });
    }
}

// Event
#[derive(Clone, Debug, EventTs)]
pub struct Started {
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub id: uuid::Uuid,
    pub file_name: String,
    pub file_id: uuid::Uuid,
    pub parent_id: Option<uuid::Uuid>,
    pub owner_id: uuid::Uuid,
}

impl Event for Started {
    type Aggregate = upload::Upload;

    fn apply(&self, _aggregate: Self::Aggregate) -> Self::Aggregate {
        return Self::Aggregate {
            id: self.id,
            created_at: self.timestamp,
            updated_at: self.timestamp,
            version: 0,
            file_name: self.file_name.clone(),
            file_id: self.file_id,
            parent_id: self.parent_id,
            size: 0,
            type_: String::new(),
            owner_id: self.owner_id,
        };
    }
}
