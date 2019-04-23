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
use drive::domain::File;


#[derive(Clone, Debug)]
pub struct RemoveFiles {
    pub files: Vec<uuid::Uuid>,
    pub owner_id: uuid::Uuid,
    pub metadata: EventMetadata,
}

impl eventsourcing::Command for RemoveFiles {
    type Aggregate = playlist::Playlist;
    type Event = playlist::Event;
    type Context = PooledConnection<ConnectionManager<PgConnection>>;
    type Error = KernelError;
    type NonStoredData = ();

    fn validate(&self, ctx: &Self::Context, aggregate: &Self::Aggregate) -> Result<(), Self::Error> {
        use kernel::db::schema::{
            drive_files,
            music_playlists_files,
        };
        use diesel::prelude::*;
        use diesel::pg::expression::dsl::any;

        // check that file is owned by same owner
        let files: Vec<File> = drive_files::dsl::drive_files
            .filter(drive_files::dsl::owner_id.eq(self.owner_id))
            .filter(drive_files::dsl::deleted_at.is_null())
            .filter(drive_files::dsl::trashed_at.is_null())
            .filter(drive_files::dsl::id.eq(any(&self.files)))
            .load(ctx)?;

        if files.len() != self.files.len() {
            return Err(KernelError::Validation("File not found.".to_string()));
        }

        // check that files are already in playlist
        let already_in_playlist: i64 = music_playlists_files::dsl::music_playlists_files
            .filter(music_playlists_files::dsl::playlist_id.eq(aggregate.id))
            .filter(music_playlists_files::dsl::file_id.eq(any(&self.files)))
            .count()
            .get_result(ctx)?;

        if already_in_playlist as usize!= self.files.len() {
            return Err(KernelError::Validation("File is not in in playlist.".to_string()));
        }

        return Ok(());
    }

    fn build_event(&self, ctx: &Self::Context, aggregate: &Self::Aggregate) -> Result<(Self::Event, Self::NonStoredData), Self::Error> {
        use kernel::db::schema::{
            music_playlists_files,
        };
        use diesel::prelude::*;
        use diesel::pg::expression::dsl::any;


        diesel::delete(music_playlists_files::dsl::music_playlists_files
            .filter(music_playlists_files::dsl::file_id.eq(any(&self.files)))
        )
            .execute(ctx)?;

        let data = playlist::EventData::FilesRemovedV1(playlist::FilesRemovedV1{
            files: self.files.clone(),
        });

        return  Ok((playlist::Event{
            id: uuid::Uuid::new_v4(),
            timestamp: chrono::Utc::now(),
            data,
            aggregate_id: aggregate.id,
            metadata: self.metadata.clone(),
        }, ()));
    }
}
