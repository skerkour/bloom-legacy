use crate::domain::{note, Note};
use actix::{Handler, Message};
use kernel::{db::DbActor, events::EventMetadata, KernelError};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateNote {
    pub title: String,
    pub body: String,
    pub account_id: uuid::Uuid,
    pub request_id: uuid::Uuid,
    pub session_id: uuid::Uuid,
}

impl Message for CreateNote {
    type Result = Result<Note, KernelError>;
}

impl Handler<CreateNote> for DbActor {
    type Result = Result<Note, KernelError>;

    fn handle(&mut self, msg: CreateNote, _: &mut Self::Context) -> Self::Result {
        use diesel::prelude::*;
        use kernel::db::schema::{notes_notes, notes_notes_events};

        let conn = self.pool.get().map_err(|_| KernelError::R2d2)?;

        return Ok(conn.transaction::<_, KernelError, _>(|| {
            // create Note
            let metadata = EventMetadata {
                actor_id: Some(msg.account_id),
                request_id: Some(msg.request_id),
                session_id: Some(msg.session_id),
            };
            let create_cmd = note::Create {
                title: msg.title,
                body: msg.body,
                owner_id: msg.account_id,
                metadata,
            };
            let (note, event, _) = eventsourcing::execute(&conn, Note::new(), &create_cmd)?;

            diesel::insert_into(notes_notes::dsl::notes_notes)
                .values(&note)
                .execute(&conn)?;
            diesel::insert_into(notes_notes_events::dsl::notes_notes_events)
                .values(&event)
                .execute(&conn)?;

            return Ok(note);
        })?);
    }
}
