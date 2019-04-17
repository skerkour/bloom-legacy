use diesel::{
    PgConnection,
    r2d2::{PooledConnection, ConnectionManager},
};
use kernel::{
    KernelError,
    events::EventMetadata,
};
use std::collections::HashSet;
use crate::{
    domain::album,
    validators,
};
use drive::domain::File;


#[derive(Clone, Debug)]
pub struct AddFiles {
    pub files: Vec<uuid::Uuid>,
    pub owner_id: uuid::Uuid,
    pub metadata: EventMetadata,
}

impl eventsourcing::Command for AddFiles {
    type Aggregate = album::Album;
    type Event = album::Event;
    type Context = PooledConnection<ConnectionManager<PgConnection>>;
    type Error = KernelError;
    type NonStoredData = ();

    fn validate(&self, ctx: &Self::Context, aggregate: &Self::Aggregate) -> Result<(), Self::Error> {
        use kernel::db::schema::{
            gallery_albums,
            drive_files,
            gallery_albums_files,
        };
        use diesel::prelude::*;
        use diesel::pg::expression::dsl::any;

        let mut valid_types = HashSet::new();
        // Add some books.
        valid_types.insert("image/gif".to_string());
        valid_types.insert("image/png".to_string());
        valid_types.insert("image/jpeg".to_string());

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

        // check that files is not already in album
        let already_in_album: i64 = gallery_albums_files::dsl::gallery_albums_files
            .filter(gallery_albums_files::dsl::album_id.eq(aggregate.id))
            .filter(gallery_albums_files::dsl::file_id.eq(any(&self.files)))
            .count()
            .get_result(ctx)?;

        if already_in_album >= 1 {
            return Err(KernelError::Validation("File is already in album.".to_string()));
        }

        // check that file is good mimetype: TODO
        for file in files {
            if !valid_types.contains(&file.type_) {
                return Err(KernelError::Validation("File type is not valid.".to_string()));
            }
        }

        return Ok(());
    }

    fn build_event(&self, ctx: &Self::Context, aggregate: &Self::Aggregate) -> Result<(Self::Event, Self::NonStoredData), Self::Error> {
        use kernel::db::schema::{
            gallery_albums_files::dsl::gallery_albums_files,
        };
        use diesel::prelude::*;

        let files: Vec<album::AlbumFile> = self.files.iter().map(|file_id|
            album::AlbumFile{
                id: uuid::Uuid::new_v4(),
                album_id: aggregate.id,
                file_id: *file_id,
            }
        ).collect();

        diesel::insert_into(gallery_albums_files)
            .values(&files)
            .execute(ctx)?;

        let data = album::EventData::FilesAddedV1(album::FilesAddedV1{
            files: self.files.clone(),
        });

        return  Ok((album::Event{
            id: uuid::Uuid::new_v4(),
            timestamp: chrono::Utc::now(),
            data,
            aggregate_id: aggregate.id,
            metadata: self.metadata.clone(),
        }, ()));
    }
}
