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
    validators,
};
use drive::domain::File;


#[derive(Clone, Debug)]
pub struct AddItem {
    pub file_id: uuid::Uuid,
    pub owner_id: uuid::Uuid,
    pub metadata: EventMetadata,
}

impl eventsourcing::Command for AddItem {
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

        // check that file is owned by same owner
        let _: File = drive_files::dsl::drive_files
            .filter(drive_files::dsl::owner_id.eq(self.owner_id))
            .filter(drive_files::dsl::deleted_at.is_null())
            .filter(drive_files::dsl::trashed_at.is_null())
            .filter(drive_files::dsl::id.eq(self.file_id))
            .first(ctx)?;

        // check that file is not already in album
        let already_in_album: i64 = gallery_albums_items::dsl::gallery_albums_items
            .filter(gallery_albums_items::dsl::album_id.eq(aggregate.id))
            .filter(gallery_albums_items::dsl::file_id.eq(self.file_id))
            .count()
            .get_result(ctx)?;

        if already_in_album >= 1 {
            return Err(KernelError::Validation("File is already in album.".to_string()));
        }

        // check that file is good mimetype: TODO

        return Ok(());
    }

    fn build_event(&self, ctx: &Self::Context, aggregate: &Self::Aggregate) -> Result<(Self::Event, Self::NonStoredData), Self::Error> {
        use kernel::db::schema::{
            gallery_albums_items::dsl::gallery_albums_items,
        };
        use diesel::prelude::*;

        let item = album::AlbumItem{
            id: uuid::Uuid::new_v4(),
            album_id: aggregate.id,
            file_id: self.file_id,
        };

        diesel::insert_into(gallery_albums_items)
            .values(&item)
            .execute(ctx)?;

        let data = album::EventData::ItemAddedV1(album::ItemAddedV1{
            file_id: self.file_id,
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
