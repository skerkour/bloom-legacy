use serde::{Serialize, Deserialize};
use diesel::{
    PgConnection,
    r2d2::{PooledConnection, ConnectionManager},
};
use kernel::{
    KernelError,
    events::EventMetadata,
};
use crate::{
    domain::album,
};


#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Delete {
    pub metadata: EventMetadata,
}


impl eventsourcing::Command for Delete {
    type Aggregate = album::Album;
    type Event = album::Event;
    type Context = PooledConnection<ConnectionManager<PgConnection>>;
    type Error = KernelError;
    type NonStoredData = ();

    fn validate(&self, _ctx: &Self::Context, aggregate: &Self::Aggregate) -> Result<(), Self::Error> {
        if aggregate.deleted_at.is_some() {
            return Err(KernelError::NotFound("Album not found".to_string()));
        }

        return Ok(());
    }

    // TODO: improve
    fn build_event(&self, ctx: &Self::Context, aggregate: &Self::Aggregate) -> Result<(Self::Event, Self::NonStoredData), Self::Error> {
        use kernel::db::schema::{
            drive_files,
            gallery_albums_files,
        };
        use diesel::prelude::*;
        use diesel::pg::expression::dsl::any;

        let files: Vec<uuid::Uuid> = drive_files::dsl::drive_files
            .inner_join(gallery_albums_files::table)
            .filter(gallery_albums_files::dsl::album_id.eq(aggregate.id))
            .filter(drive_files::dsl::deleted_at.is_null())
            .filter(drive_files::dsl::trashed_at.is_null())
            .select(drive_files::id)
            .load(ctx)?;

        diesel::delete(gallery_albums_files::dsl::gallery_albums_files
            .filter(gallery_albums_files::dsl::file_id.eq(any(&files)))
        )
            .execute(ctx)?;

        return  Ok((album::Event{
            id: uuid::Uuid::new_v4(),
            timestamp: chrono::Utc::now(),
            data: album::EventData::DeletedV1,
            aggregate_id: aggregate.id,
            metadata: self.metadata.clone(),
        }, ()));
    }
}
