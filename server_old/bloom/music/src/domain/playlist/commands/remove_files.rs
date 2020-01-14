use crate::domain::playlist;
use diesel::{
    r2d2::{ConnectionManager, PooledConnection},
    PgConnection,
};
use drive::domain::File;
use eventsourcing::{Event, EventTs};
use kernel::KernelError;

#[derive(Clone, Debug)]
pub struct RemoveFiles {
    pub files: Vec<uuid::Uuid>,
    pub owner_id: uuid::Uuid,
}

impl eventsourcing::Command for RemoveFiles {
    type Aggregate = playlist::Playlist;
    type Event = FilesRemoved;
    type Context = PooledConnection<ConnectionManager<PgConnection>>;
    type Error = KernelError;

    fn validate(
        &self,
        ctx: &Self::Context,
        aggregate: &Self::Aggregate,
    ) -> Result<(), Self::Error> {
        use diesel::pg::expression::dsl::any;
        use diesel::prelude::*;
        use kernel::db::schema::{drive_files, music_playlists_files};

        // check that file is owned by same owner
        let files: Vec<File> = drive_files::dsl::drive_files
            .filter(drive_files::dsl::owner_id.eq(self.owner_id))
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

        if already_in_playlist as usize != self.files.len() {
            return Err(KernelError::Validation(
                "File is not in in playlist.".to_string(),
            ));
        }

        return Ok(());
    }

    fn build_event(
        &self,
        ctx: &Self::Context,
        _aggregate: &Self::Aggregate,
    ) -> Result<Self::Event, Self::Error> {
        use diesel::pg::expression::dsl::any;
        use diesel::prelude::*;
        use kernel::db::schema::music_playlists_files;

        diesel::delete(
            music_playlists_files::dsl::music_playlists_files
                .filter(music_playlists_files::dsl::file_id.eq(any(&self.files))),
        )
        .execute(ctx)?;

        return Ok(FilesRemoved {
            timestamp: chrono::Utc::now(),
            files: self.files.clone(),
        });
    }
}

// Event
#[derive(Clone, Debug, EventTs)]
pub struct FilesRemoved {
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub files: Vec<uuid::Uuid>,
}

impl Event for FilesRemoved {
    type Aggregate = playlist::Playlist;

    fn apply(&self, aggregate: Self::Aggregate) -> Self::Aggregate {
        return aggregate;
    }
}
