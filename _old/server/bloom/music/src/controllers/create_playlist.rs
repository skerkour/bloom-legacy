use crate::domain::{playlist, Playlist};
use actix::{Handler, Message};
use kernel::{db::DbActor, KernelError};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreatePlaylist {
    pub name: String,
    pub account_id: uuid::Uuid,
    pub request_id: uuid::Uuid,
    pub session_id: uuid::Uuid,
}

impl Message for CreatePlaylist {
    type Result = Result<Playlist, KernelError>;
}

impl Handler<CreatePlaylist> for DbActor {
    type Result = Result<Playlist, KernelError>;

    fn handle(&mut self, msg: CreatePlaylist, _: &mut Self::Context) -> Self::Result {
        use diesel::prelude::*;
        use kernel::db::schema::music_playlists;

        let conn = self.pool.get().map_err(|_| KernelError::R2d2)?;

        return Ok(conn.transaction::<_, KernelError, _>(|| {
            // create Playlist
            let create_cmd = playlist::Create {
                name: msg.name,
                owner_id: msg.account_id,
            };
            let (playlist, _) = eventsourcing::execute(&conn, Playlist::new(), &create_cmd)?;

            diesel::insert_into(music_playlists::dsl::music_playlists)
                .values(&playlist)
                .execute(&conn)?;

            return Ok(playlist);
        })?);
    }
}
