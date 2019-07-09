use crate::domain::album;
use diesel::{
    r2d2::{ConnectionManager, PooledConnection},
    PgConnection,
};
use drive::domain::File;
use eventsourcing::{Event, EventTs};
use kernel::KernelError;
use std::collections::HashSet;

#[derive(Clone, Debug)]
pub struct AddFiles {
    pub files: Vec<uuid::Uuid>,
    pub owner_id: uuid::Uuid,
}

impl eventsourcing::Command for AddFiles {
    type Aggregate = album::Album;
    type Event = FilesAdded;
    type Context = PooledConnection<ConnectionManager<PgConnection>>;
    type Error = KernelError;

    fn validate(
        &self,
        ctx: &Self::Context,
        aggregate: &Self::Aggregate,
    ) -> Result<(), Self::Error> {
        use diesel::pg::expression::dsl::any;
        use diesel::prelude::*;
        use kernel::db::schema::{drive_files, gallery_albums_files};

        let mut valid_types = HashSet::new();
        valid_types.insert("image/gif".to_string());
        valid_types.insert("image/png".to_string());
        valid_types.insert("image/jpeg".to_string());

        // check that file is owned by same owner
        let files: Vec<File> = drive_files::dsl::drive_files
            .filter(drive_files::dsl::owner_id.eq(self.owner_id))
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
            return Err(KernelError::Validation(
                "File is already in album.".to_string(),
            ));
        }

        // check that file is good mimetype: TODO
        for file in files {
            if !valid_types.contains(&file.type_) {
                return Err(KernelError::Validation(
                    "File type is not valid.".to_string(),
                ));
            }
        }

        return Ok(());
    }

    fn build_event(
        &self,
        ctx: &Self::Context,
        aggregate: &Self::Aggregate,
    ) -> Result<Self::Event, Self::Error> {
        use diesel::prelude::*;
        use kernel::db::schema::gallery_albums_files::dsl::gallery_albums_files;

        let files: Vec<album::AlbumFile> = self
            .files
            .iter()
            .map(|file_id| album::AlbumFile {
                id: uuid::Uuid::new_v4(),
                album_id: aggregate.id,
                file_id: *file_id,
            })
            .collect();

        diesel::insert_into(gallery_albums_files)
            .values(&files)
            .execute(ctx)?;

        return Ok(FilesAdded {
            timestamp: chrono::Utc::now(),
            files: self.files.clone(),
        });
    }
}

// Event
#[derive(Clone, Debug, EventTs)]
pub struct FilesAdded {
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub files: Vec<uuid::Uuid>,
}

impl Event for FilesAdded {
    type Aggregate = album::Album;

    fn apply(&self, aggregate: Self::Aggregate) -> Self::Aggregate {
        return aggregate;
    }
}
