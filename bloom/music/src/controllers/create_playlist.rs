use actix::{Message, Handler};
use serde::{Serialize, Deserialize};
use kernel::{
    KernelError,
    events::EventMetadata,
    db::DbActor
};
use crate::domain::{
    Playlist,
    playlist,
};


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
        use kernel::db::schema::{
            music_playlists,
            music_playlists_events,
        };
        use diesel::prelude::*;


        let conn = self.pool.get()
            .map_err(|_| KernelError::R2d2)?;

        return Ok(conn.transaction::<_, KernelError, _>(|| {

            // create Playlist
            let metadata = EventMetadata{
                actor_id: Some(msg.account_id),
                request_id: Some(msg.request_id),
                session_id: Some(msg.session_id),
            };
            let create_cmd = playlist::Create{
                name: msg.name,
                owner_id: msg.account_id,
                metadata,
            };
            let (playlist, event, _) = eventsourcing::execute(&conn, Playlist::new(), &create_cmd)?;

            diesel::insert_into(music_playlists::dsl::music_playlists)
                .values(&playlist)
                .execute(&conn)?;
            diesel::insert_into(music_playlists_events::dsl::music_playlists_events)
                .values(&event)
                .execute(&conn)?;

            return Ok(playlist);
        })?);
    }
}
