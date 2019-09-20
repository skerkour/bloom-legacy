use crate::domain::{album, Album};
use actix::{Handler, Message};
use kernel::{db::DbActor, KernelError};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RenameAlbum {
    pub album_id: uuid::Uuid,
    pub name: String,
    pub account_id: uuid::Uuid,
    pub session_id: uuid::Uuid,
    pub request_id: uuid::Uuid,
}

impl Message for RenameAlbum {
    type Result = Result<Album, KernelError>;
}

impl Handler<RenameAlbum> for DbActor {
    type Result = Result<Album, KernelError>;

    fn handle(&mut self, msg: RenameAlbum, _: &mut Self::Context) -> Self::Result {
        use diesel::prelude::*;
        use kernel::db::schema::gallery_albums;

        let conn = self.pool.get().map_err(|_| KernelError::R2d2)?;

        return Ok(conn.transaction::<_, KernelError, _>(|| {
            let rename_cmd = album::Rename { name: msg.name };

            let album_to_update: Album = gallery_albums::dsl::gallery_albums
                .filter(gallery_albums::dsl::id.eq(msg.album_id))
                .filter(gallery_albums::dsl::owner_id.eq(msg.account_id))
                .for_update()
                .first(&conn)?;

            let (album_to_update, _) = eventsourcing::execute(&conn, album_to_update, &rename_cmd)?;
            // update album
            diesel::update(&album_to_update)
                .set(&album_to_update)
                .execute(&conn)?;

            return Ok(album_to_update);
        })?);
    }
}
