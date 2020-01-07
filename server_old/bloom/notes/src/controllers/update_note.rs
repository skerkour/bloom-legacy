use crate::domain::{note, Note};
use actix::{Handler, Message};
use kernel::{db::DbActor, KernelError};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UpdateNote {
    pub body: Option<String>,
    pub title: Option<String>,
    pub note_id: uuid::Uuid,
    pub actor_id: uuid::Uuid,
    pub session_id: uuid::Uuid,
    pub request_id: uuid::Uuid,
}

impl Message for UpdateNote {
    type Result = Result<Note, KernelError>;
}

impl Handler<UpdateNote> for DbActor {
    type Result = Result<Note, KernelError>;

    fn handle(&mut self, msg: UpdateNote, _: &mut Self::Context) -> Self::Result {
        use diesel::prelude::*;
        use kernel::db::schema::notes_notes;

        let conn = self.pool.get().map_err(|_| KernelError::R2d2)?;

        return Ok(conn.transaction::<_, KernelError, _>(|| {
            let note_to_update: Note = notes_notes::dsl::notes_notes
                .filter(notes_notes::dsl::id.eq(msg.note_id))
                .filter(notes_notes::dsl::owner_id.eq(msg.actor_id))
                .for_update()
                .first(&conn)?;

            // title
            let note_to_update = match &msg.title {
                Some(title) if title != &note_to_update.title => {
                    let update_title_cmd = note::UpdateTitle {
                        title: title.to_string(),
                    };

                    let (note_to_update, _) =
                        eventsourcing::execute(&conn, note_to_update, &update_title_cmd)?;

                    // update note
                    diesel::update(&note_to_update)
                        .set(&note_to_update)
                        .execute(&conn)?;
                    note_to_update
                }
                _ => note_to_update,
            };

            // body
            let note_to_update = match &msg.body {
                Some(body) if body != &note_to_update.body => {
                    let update_body_cmd = note::UpdateBody {
                        body: body.to_string(),
                    };

                    let (note_to_update, _) =
                        eventsourcing::execute(&conn, note_to_update, &update_body_cmd)?;

                    // update note
                    diesel::update(&note_to_update)
                        .set(&note_to_update)
                        .execute(&conn)?;
                    note_to_update
                }
                _ => note_to_update,
            };

            return Ok(note_to_update);
        })?);
    }
}
