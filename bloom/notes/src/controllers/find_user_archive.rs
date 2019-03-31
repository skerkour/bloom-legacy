use actix::{Message, Handler};
use serde::{Serialize, Deserialize};
use kernel::{
    KernelError,
    db::DbActor,
};
use crate::domain;


#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct FindUserArchive {
    pub user_id: uuid::Uuid,
}

impl Message for FindUserArchive {
    type Result = Result<Vec<domain::Note>, KernelError>;
}

impl Handler<FindUserArchive> for DbActor {
    type Result = Result<Vec<domain::Note>, KernelError>;

    fn handle(&mut self, msg: FindUserArchive, _: &mut Self::Context) -> Self::Result {
        use kernel::db::schema::{
            notes_notes,
        };
        use diesel::prelude::*;


        let conn = self.pool.get()
            .map_err(|_| KernelError::R2d2)?;

        let notes: Vec<domain::Note> = notes_notes::dsl::notes_notes
                .filter(notes_notes::dsl::owner_id.eq(msg.user_id))
                .filter(notes_notes::dsl::deleted_at.is_null())
                .filter(notes_notes::dsl::archived_at.is_not_null())
                .filter(notes_notes::dsl::removed_at.is_null())
                .load(&conn)?;

        return Ok(notes);
    }
}
