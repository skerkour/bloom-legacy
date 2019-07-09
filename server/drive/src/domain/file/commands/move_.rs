use crate::{domain::file, FOLDER_TYPE};
use diesel::{
    r2d2::{ConnectionManager, PooledConnection},
    PgConnection,
};
use eventsourcing::{Event, EventTs};
use kernel::KernelError;

#[derive(Clone, Debug)]
pub struct Move {
    pub to: uuid::Uuid,
}

impl eventsourcing::Command for Move {
    type Aggregate = file::File;
    type Event = Moved;
    type Context = PooledConnection<ConnectionManager<PgConnection>>;
    type Error = KernelError;

    fn validate(
        &self,
        ctx: &Self::Context,
        aggregate: &Self::Aggregate,
    ) -> Result<(), Self::Error> {
        use diesel::prelude::*;
        use kernel::db::schema::drive_files;

        // fetch new parent
        let new_parent: file::File = drive_files::dsl::drive_files
            .filter(drive_files::dsl::id.eq(self.to))
            .filter(drive_files::dsl::owner_id.eq(aggregate.owner_id))
            .filter(drive_files::dsl::trashed_at.is_null())
            .first(ctx)?;

        if new_parent.type_ != FOLDER_TYPE {
            return Err(KernelError::Validation(
                "Destination must be a file".to_string(),
            ));
        }

        if new_parent.id == aggregate.id {
            return Err(KernelError::Validation(
                "Destination can't be source".to_string(),
            ));
        }

        return Ok(());
    }

    fn build_event(
        &self,
        _ctx: &Self::Context,
        _aggregate: &Self::Aggregate,
    ) -> Result<Self::Event, Self::Error> {
        return Ok(Moved {
            timestamp: chrono::Utc::now(),
            to: self.to,
        });
    }
}

// Event
#[derive(Clone, Debug, EventTs)]
pub struct Moved {
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub to: uuid::Uuid, // new parent
}

impl Event for Moved {
    type Aggregate = file::File;

    fn apply(&self, aggregate: Self::Aggregate) -> Self::Aggregate {
        return Self::Aggregate {
            parent_id: Some(self.to),
            ..aggregate
        };
    }
}
