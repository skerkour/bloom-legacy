use actix::{Message, Handler};
use serde::{Serialize, Deserialize};
use kernel::{
    db::DbActor,
    KernelError,
};
use crate::domain;


#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct FindAlbums {
    pub account_id: uuid::Uuid,
}

impl Message for FindAlbums {
    type Result = Result<Vec<domain::Album>, KernelError>;
}

impl Handler<FindAlbums> for DbActor {
    type Result = Result<Vec<domain::Album>, KernelError>;

    fn handle(&mut self, msg: FindAlbums, _: &mut Self::Context) -> Self::Result {
        use kernel::db::schema::{
            gallery_albums,
        };
        use diesel::prelude::*;


        let conn = self.pool.get()
            .map_err(|_| KernelError::R2d2)?;

        let albums: Vec<domain::Album> = gallery_albums::dsl::gallery_albums
            .filter(gallery_albums::dsl::owner_id.eq(msg.account_id))
            .filter(gallery_albums::dsl::deleted_at.is_null())
            .load(&conn)?;

        return Ok(albums);
    }
}
