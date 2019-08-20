use crate::domain::{album, Album};
use actix::{Handler, Message};
use kernel::{db::DbActor, KernelError};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateAlbum {
    pub name: String,
    pub account_id: uuid::Uuid,
    pub request_id: uuid::Uuid,
    pub session_id: uuid::Uuid,
}

impl Message for CreateAlbum {
    type Result = Result<Album, KernelError>;
}

impl Handler<CreateAlbum> for DbActor {
    type Result = Result<Album, KernelError>;

    fn handle(&mut self, msg: CreateAlbum, _: &mut Self::Context) -> Self::Result {
        use diesel::prelude::*;
        use kernel::db::schema::gallery_albums;

        let conn = self.pool.get().map_err(|_| KernelError::R2d2)?;

        return Ok(conn.transaction::<_, KernelError, _>(|| {
            // create Album
            let create_cmd = album::Create {
                name: msg.name,
                owner_id: msg.account_id,
            };
            let (album, _) = eventsourcing::execute(&conn, Album::new(), &create_cmd)?;

            diesel::insert_into(gallery_albums::dsl::gallery_albums)
                .values(&album)
                .execute(&conn)?;

            return Ok(album);
        })?);
    }
}
