use actix::{Message, Handler};
use serde::{Serialize, Deserialize};
use kernel::{
    KernelError,
    events::EventMetadata,
    db::DbActor
};
use crate::domain::{
    Album,
    album,
};


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
        use kernel::db::schema::{
            gallery_albums,
            gallery_albums_events,
        };
        use diesel::prelude::*;


        let conn = self.pool.get()
            .map_err(|_| KernelError::R2d2)?;

        return Ok(conn.transaction::<_, KernelError, _>(|| {

            // create Album
            let metadata = EventMetadata{
                actor_id: Some(msg.account_id),
                request_id: Some(msg.request_id),
                session_id: Some(msg.session_id),
            };
            let create_cmd = album::Create{
                name: msg.name,
                owner_id: msg.account_id,
                metadata,
            };
            let (album, event, _) = eventsourcing::execute(&conn, Album::new(), &create_cmd)?;

            diesel::insert_into(gallery_albums::dsl::gallery_albums)
                .values(&album)
                .execute(&conn)?;
            diesel::insert_into(gallery_albums_events::dsl::gallery_albums_events)
                .values(&event)
                .execute(&conn)?;

            return Ok(album);
        })?);
    }
}
