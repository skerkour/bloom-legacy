use crate::domain::{album, Album};
use actix::{Handler, Message};
use kernel::{db::DbActor, events::EventMetadata, KernelError};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DeleteAlbum {
    pub album_id: uuid::Uuid,
    pub account_id: uuid::Uuid,
    pub session_id: uuid::Uuid,
    pub request_id: uuid::Uuid,
}

impl Message for DeleteAlbum {
    type Result = Result<(), KernelError>;
}

impl Handler<DeleteAlbum> for DbActor {
    type Result = Result<(), KernelError>;

    fn handle(&mut self, msg: DeleteAlbum, _: &mut Self::Context) -> Self::Result {
        use diesel::prelude::*;
        use kernel::db::schema::{gallery_albums, gallery_albums_events};

        let conn = self.pool.get().map_err(|_| KernelError::R2d2)?;

        return Ok(conn.transaction::<_, KernelError, _>(|| {
            let metadata = EventMetadata {
                actor_id: Some(msg.account_id),
                request_id: Some(msg.request_id),
                session_id: Some(msg.session_id),
            };
            let delete_cmd = album::Delete { metadata };

            let album_to_update: Album = gallery_albums::dsl::gallery_albums
                .filter(gallery_albums::dsl::id.eq(msg.album_id))
                .filter(gallery_albums::dsl::owner_id.eq(msg.account_id))
                .filter(gallery_albums::dsl::deleted_at.is_null())
                .for_update()
                .first(&conn)?;

            let (album_to_update, event, _) =
                eventsourcing::execute(&conn, album_to_update, &delete_cmd)?;
            // update album
            diesel::update(&album_to_update)
                .set(&album_to_update)
                .execute(&conn)?;
            diesel::insert_into(gallery_albums_events::dsl::gallery_albums_events)
                .values(&event)
                .execute(&conn)?;

            return Ok(());
        })?);
    }
}
