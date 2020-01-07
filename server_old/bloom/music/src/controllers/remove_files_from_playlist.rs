use crate::domain::{playlist, Playlist};
use actix::{Handler, Message};
use kernel::{db::DbActor, KernelError};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoveFilesFromPlaylist {
    pub playlist_id: uuid::Uuid,
    pub files: Vec<uuid::Uuid>,
    pub account_id: uuid::Uuid,
    pub session_id: uuid::Uuid,
    pub request_id: uuid::Uuid,
}

impl Message for RemoveFilesFromPlaylist {
    type Result = Result<Playlist, KernelError>;
}

impl Handler<RemoveFilesFromPlaylist> for DbActor {
    type Result = Result<Playlist, KernelError>;

    fn handle(&mut self, msg: RemoveFilesFromPlaylist, _: &mut Self::Context) -> Self::Result {
        use diesel::prelude::*;
        use kernel::db::schema::music_playlists;

        let conn = self.pool.get().map_err(|_| KernelError::R2d2)?;

        return Ok(conn.transaction::<_, KernelError, _>(|| {
            let remove_cmd = playlist::RemoveFiles {
                files: msg.files.clone(),
                owner_id: msg.account_id,
            };

            let playlist_to_update: Playlist = music_playlists::dsl::music_playlists
                .filter(music_playlists::dsl::id.eq(msg.playlist_id))
                .filter(music_playlists::dsl::owner_id.eq(msg.account_id))
                .for_update()
                .first(&conn)?;

            let (playlist_to_update, _) =
                eventsourcing::execute(&conn, playlist_to_update, &remove_cmd)?;
            // update playlist
            diesel::update(&playlist_to_update)
                .set(&playlist_to_update)
                .execute(&conn)?;

            return Ok(playlist_to_update);
        })?);
    }
}
