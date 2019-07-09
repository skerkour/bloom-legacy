use crate::domain;
use actix::{Handler, Message};
use kernel::{db::DbActor, KernelError};
use serde::{Deserialize, Serialize};

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
        use diesel::prelude::*;
        use kernel::db::schema::gallery_albums;

        let conn = self.pool.get().map_err(|_| KernelError::R2d2)?;

        let albums: Vec<domain::Album> = gallery_albums::dsl::gallery_albums
            .filter(gallery_albums::dsl::owner_id.eq(msg.account_id))
            .load(&conn)?;

        return Ok(albums);
    }
}
