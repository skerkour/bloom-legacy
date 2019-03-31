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
pub struct ArchiveNote {
    pub note_id: uuid::Uuid,
    pub user_id: uuid::Uuid,
    pub session_id: uuid::Uuid,
    pub request_id: uuid::Uuid,
}

impl Message for ArchiveNote {
    type Result = Result<(), KernelError>;
}

impl Handler<ArchiveNote> for DbActor {
    type Result = Result<(), KernelError>;

    fn handle(&mut self, msg: ArchiveNote, _: &mut Self::Context) -> Self::Result {
        use kernel::db::schema::{
            notes_notes,
            notes_notes_events,
        };
        use diesel::prelude::*;


        let conn = self.pool.get()
            .map_err(|_| KernelError::R2d2)?;

        return Ok(conn.transaction::<_, KernelError, _>(|| {
            let metadata = EventMetadata{
                actor_id: Some(msg.user_id),
                request_id: Some(msg.request_id),
                session_id: Some(msg.session_id),
            };
            let archive_cmd = note::Archive{
                metadata,
            };

            let note_to_update: Note = notes_notes::dsl::notes_notes
                .filter(notes_notes::dsl::id.eq(msg.note_id))
                .filter(notes_notes::dsl::owner_id.eq(msg.user_id))
                .filter(notes_notes::dsl::deleted_at.is_null())
                .for_update()
                .first(&conn)?;

            let (note_to_update, event, _) = eventsourcing::execute(&conn, note_to_update, &archive_cmd)?;

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
