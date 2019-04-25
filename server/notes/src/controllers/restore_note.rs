use actix::{Message, Handler};
use serde::{Serialize, Deserialize};
use kernel::{
    db::DbActor,
    events::EventMetadata,
    KernelError,
};
use crate::domain::{
    note,
    Note,
};


#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RestoreNote {
    pub note_id: uuid::Uuid,
    pub actor_id: uuid::Uuid,
    pub session_id: uuid::Uuid,
    pub request_id: uuid::Uuid,
}

impl Message for RestoreNote {
    type Result = Result<(), KernelError>;
}

impl Handler<RestoreNote> for DbActor {
    type Result = Result<(), KernelError>;

    fn handle(&mut self, msg: RestoreNote, _: &mut Self::Context) -> Self::Result {
        use kernel::db::schema::{
            notes_notes,
            notes_notes_events,
        };
        use diesel::prelude::*;


        let conn = self.pool.get()
            .map_err(|_| KernelError::R2d2)?;

        return Ok(conn.transaction::<_, KernelError, _>(|| {
            let metadata = EventMetadata{
                actor_id: Some(msg.actor_id),
                request_id: Some(msg.request_id),
                session_id: Some(msg.session_id),
            };
            let restore_cmd = note::Restore{
                metadata,
            };

            let note_to_update: Note = notes_notes::dsl::notes_notes
                .filter(notes_notes::dsl::id.eq(msg.note_id))
                .filter(notes_notes::dsl::owner_id.eq(msg.actor_id))
                .filter(notes_notes::dsl::deleted_at.is_null())
                .for_update()
                .first(&conn)?;

            let (note_to_update, event, _) = eventsourcing::execute(&conn, note_to_update, &restore_cmd)?;

            // update note
            diesel::update(&note_to_update)
                .set(&note_to_update)
                .execute(&conn)?;
            diesel::insert_into(notes_notes_events::dsl::notes_notes_events)
                .values(&event)
                .execute(&conn)?;

            return Ok(());
        })?);
    }
}
