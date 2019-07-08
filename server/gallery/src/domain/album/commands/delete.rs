use crate::domain::album;
use diesel::{
    r2d2::{ConnectionManager, PooledConnection},
    PgConnection,
};
use eventsourcing::{Event, EventTs};
use kernel::KernelError;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Delete {}

impl eventsourcing::Command for Delete {
    type Aggregate = album::Album;
    type Event = Deleted;
    type Context = PooledConnection<ConnectionManager<PgConnection>>;
    type Error = KernelError;

    fn validate(
        &self,
        _ctx: &Self::Context,
        aggregate: &Self::Aggregate,
    ) -> Result<(), Self::Error> {
        if aggregate.deleted_at.is_some() {
            return Err(KernelError::NotFound("Album not found".to_string()));
        }

        return Ok(());
    }

    // TODO: improve
    fn build_event(
        &self,
        ctx: &Self::Context,
        aggregate: &Self::Aggregate,
    ) -> Result<Self::Event, Self::Error> {
        use diesel::pg::expression::dsl::any;
        use diesel::prelude::*;
        use kernel::db::schema::{drive_files, gallery_albums_files};

        let files: Vec<uuid::Uuid> = drive_files::dsl::drive_files
            .inner_join(gallery_albums_files::table)
            .filter(gallery_albums_files::dsl::album_id.eq(aggregate.id))
            .filter(drive_files::dsl::deleted_at.is_null())
            .filter(drive_files::dsl::trashed_at.is_null())
            .select(drive_files::id)
            .load(ctx)?;

        diesel::delete(
            gallery_albums_files::dsl::gallery_albums_files
                .filter(gallery_albums_files::dsl::file_id.eq(any(&files))),
        )
        .execute(ctx)?;

        return Ok(Deleted {
            timestamp: chrono::Utc::now(),
        });
    }
}

// Event
#[derive(Clone, Debug, EventTs)]
pub struct Deleted {
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

impl Event for Deleted {
    type Aggregate = album::Album;

    fn apply(&self, aggregate: Self::Aggregate) -> Self::Aggregate {
        return Self::Aggregate {
            deleted_at: Some(self.timestamp),
            ..aggregate
        };
    }
}
