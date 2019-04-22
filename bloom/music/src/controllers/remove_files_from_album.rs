use actix::{Message, Handler};
use serde::{Serialize, Deserialize};
use kernel::{
    KernelError,
    events::EventMetadata,
    db::DbActor,
};
use crate::domain::{
    album,
    Album,
};


#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoveFilesFromAlbum {
    pub album_id: uuid::Uuid,
    pub files: Vec<uuid::Uuid>,
    pub account_id: uuid::Uuid,
    pub session_id: uuid::Uuid,
    pub request_id: uuid::Uuid,
}

impl Message for RemoveFilesFromAlbum {
    type Result = Result<Album, KernelError>;
}

impl Handler<RemoveFilesFromAlbum> for DbActor {
    type Result = Result<Album, KernelError>;

    fn handle(&mut self, msg: RemoveFilesFromAlbum, _: &mut Self::Context) -> Self::Result {
        use kernel::db::schema::{
            gallery_albums,
            gallery_albums_events,
        };
        use diesel::prelude::*;


        let conn = self.pool.get()
            .map_err(|_| KernelError::R2d2)?;

        return Ok(conn.transaction::<_, KernelError, _>(|| {
            let metadata = EventMetadata{
                actor_id: Some(msg.account_id),
                request_id: Some(msg.request_id),
                session_id: Some(msg.session_id),
            };
            let remove_cmd = album::RemoveFiles{
                files: msg.files.clone(),
                owner_id: msg.account_id,
                metadata,
            };

            let album_to_update: Album = gallery_albums::dsl::gallery_albums
                .filter(gallery_albums::dsl::id.eq(msg.album_id))
                .filter(gallery_albums::dsl::owner_id.eq(msg.account_id))
                .filter(gallery_albums::dsl::deleted_at.is_null())
                .for_update()
                .first(&conn)?;

            let (album_to_update, event, _) = eventsourcing::execute(&conn, album_to_update, &remove_cmd)?;
            // update album
            diesel::update(&album_to_update)
                .set(&album_to_update)
                .execute(&conn)?;
            diesel::insert_into(gallery_albums_events::dsl::gallery_albums_events)
                .values(&event)
                .execute(&conn)?;

            return Ok(album_to_update);
        })?);
    }
}
