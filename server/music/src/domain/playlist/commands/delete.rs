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
    domain::playlist,
};


#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Delete {
    pub metadata: EventMetadata,
}


impl eventsourcing::Command for Delete {
    type Aggregate = playlist::Playlist;
    type Event = playlist::Event;
    type Context = PooledConnection<ConnectionManager<PgConnection>>;
    type Error = KernelError;
    type NonStoredData = ();

    fn validate(&self, _ctx: &Self::Context, aggregate: &Self::Aggregate) -> Result<(), Self::Error> {
        if aggregate.deleted_at.is_some() {
            return Err(KernelError::NotFound("Playlist not found".to_string()));
        }

        return Ok(());
    }

    // TODO: improve
    fn build_event(&self, ctx: &Self::Context, aggregate: &Self::Aggregate) -> Result<(Self::Event, Self::NonStoredData), Self::Error> {
        use kernel::db::schema::{
            drive_files,
            music_playlists_files,
        };
        use diesel::prelude::*;
        use diesel::pg::expression::dsl::any;

        let files: Vec<uuid::Uuid> = drive_files::dsl::drive_files
            .inner_join(music_playlists_files::table)
            .filter(music_playlists_files::dsl::playlist_id.eq(aggregate.id))
            .filter(drive_files::dsl::deleted_at.is_null())
            .filter(drive_files::dsl::trashed_at.is_null())
            .select(drive_files::id)
            .load(ctx)?;

        diesel::delete(music_playlists_files::dsl::music_playlists_files
            .filter(music_playlists_files::dsl::file_id.eq(any(&files)))
        )
            .execute(ctx)?;

        return  Ok((playlist::Event{
            id: uuid::Uuid::new_v4(),
            timestamp: chrono::Utc::now(),
            data: playlist::EventData::DeletedV1,
            aggregate_id: aggregate.id,
            metadata: self.metadata.clone(),
        }, ()));
    }
}
