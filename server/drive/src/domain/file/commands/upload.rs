use crate::domain::file;
use diesel::{
    r2d2::{ConnectionManager, PooledConnection},
    PgConnection,
};
use eventsourcing::{Event, EventTs};
use kernel::KernelError;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Upload {
    pub id: uuid::Uuid,
    pub name: String,
    pub parent_id: Option<uuid::Uuid>,
    pub size: i64,
    #[serde(rename = "type")]
    pub type_: String, // MIME type
    pub owner_id: uuid::Uuid,
}

impl eventsourcing::Command for Upload {
    type Aggregate = file::File;
    type Event = Uploaded;
    type Context = PooledConnection<ConnectionManager<PgConnection>>;
    type Error = KernelError;

    fn validate(
        &self,
        _ctx: &Self::Context,
        _aggregate: &Self::Aggregate,
    ) -> Result<(), Self::Error> {
        // TODO
        return Ok(());
    }

    fn build_event(
        &self,
        _ctx: &Self::Context,
        _aggregate: &Self::Aggregate,
    ) -> Result<Self::Event, Self::Error> {
        return Ok(Uploaded {
            timestamp: chrono::Utc::now(),
            id: self.id,
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
pub struct Uploaded {
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub id: uuid::Uuid,
    pub name: String,
    pub parent_id: Option<uuid::Uuid>,
    pub size: i64,
    pub type_: String, // MIME type
    pub owner_id: uuid::Uuid,
}

impl Event for Uploaded {
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
