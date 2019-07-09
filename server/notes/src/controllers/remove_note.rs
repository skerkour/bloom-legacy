use crate::domain::{note, Note};
use actix::{Handler, Message};
use kernel::{db::DbActor, KernelError};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoveNote {
    pub note_id: uuid::Uuid,
    pub actor_id: uuid::Uuid,
    pub session_id: uuid::Uuid,
    pub request_id: uuid::Uuid,
}

impl Message for RemoveNote {
    type Result = Result<(), KernelError>;
}

impl Handler<RemoveNote> for DbActor {
    type Result = Result<(), KernelError>;

    fn handle(&mut self, msg: RemoveNote, _: &mut Self::Context) -> Self::Result {
        use diesel::prelude::*;
        use kernel::db::schema::notes_notes;

        let conn = self.pool.get().map_err(|_| KernelError::R2d2)?;

        return Ok(conn.transaction::<_, KernelError, _>(|| {
            let remove_cmd = note::Remove {};

            let note_to_update: Note = notes_notes::dsl::notes_notes
                .filter(notes_notes::dsl::id.eq(msg.note_id))
                .filter(notes_notes::dsl::owner_id.eq(msg.actor_id))
                .for_update()
                .first(&conn)?;

            let (note_to_update, _) = eventsourcing::execute(&conn, note_to_update, &remove_cmd)?;

            // update note
            diesel::update(&note_to_update)
                .set(&note_to_update)
                .execute(&conn)?;

            return Ok(());
        })?);
    }
}
