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
pub struct RemoveFiles {
    pub files: Vec<uuid::Uuid>,
    pub owner_id: uuid::Uuid,
    pub metadata: EventMetadata,
}

impl eventsourcing::Command for RemoveFiles {
    type Aggregate = album::Album;
    type Event = album::Event;
    type Context = PooledConnection<ConnectionManager<PgConnection>>;
    type Error = KernelError;
    type NonStoredData = ();

    fn validate(&self, ctx: &Self::Context, aggregate: &Self::Aggregate) -> Result<(), Self::Error> {
        use kernel::db::schema::{
            gallery_albums,
            drive_files,
            gallery_albums_items,
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

        // check that files are already in album
        let already_in_album: i64 = gallery_albums_items::dsl::gallery_albums_items
            .filter(gallery_albums_items::dsl::album_id.eq(aggregate.id))
            .filter(gallery_albums_items::dsl::file_id.eq(any(&self.files)))
            .count()
            .get_result(ctx)?;

        if already_in_album as usize!= self.files.len() {
            return Err(KernelError::Validation("File is not in in album.".to_string()));
        }

        return Ok(());
    }

    fn build_event(&self, ctx: &Self::Context, aggregate: &Self::Aggregate) -> Result<(Self::Event, Self::NonStoredData), Self::Error> {
        use kernel::db::schema::{
            gallery_albums_items,
        };
        use diesel::prelude::*;
        use diesel::pg::expression::dsl::any;


        diesel::delete(gallery_albums_items::dsl::gallery_albums_items
            .filter(gallery_albums_items::dsl::file_id.eq(any(&self.files)))
        )
            .execute(ctx)?;

        let data = album::EventData::FilesRemovedV1(album::FilesRemovedV1{
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
