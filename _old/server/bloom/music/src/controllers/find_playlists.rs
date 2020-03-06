use crate::domain;
use actix::{Handler, Message};
use kernel::{db::DbActor, KernelError};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct FindPlaylists {
    pub account_id: uuid::Uuid,
}

impl Message for FindPlaylists {
    type Result = Result<Vec<domain::Playlist>, KernelError>;
}

impl Handler<FindPlaylists> for DbActor {
    type Result = Result<Vec<domain::Playlist>, KernelError>;

    fn handle(&mut self, msg: FindPlaylists, _: &mut Self::Context) -> Self::Result {
        use diesel::prelude::*;
        use kernel::db::schema::music_playlists;

        let conn = self.pool.get().map_err(|_| KernelError::R2d2)?;

        let playlists: Vec<domain::Playlist> = music_playlists::dsl::music_playlists
            .filter(music_playlists::dsl::owner_id.eq(msg.account_id))
            .load(&conn)?;

        return Ok(playlists);
    }
}
