use actix::{Message, Handler};
use crate::{
    db::DbActor,
    services::notes::domain,
    error::KernelError,
};
use serde::{Serialize, Deserialize};


#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct FindUserTrash {
    pub account_id: uuid::Uuid,
}

impl Message for FindUserTrash {
    type Result = Result<Vec<domain::Note>, KernelError>;
}

impl Handler<FindUserTrash> for DbActor {
    type Result = Result<Vec<domain::Note>, KernelError>;

    fn handle(&mut self, msg: FindUserTrash, _: &mut Self::Context) -> Self::Result {
        use crate::db::schema::{
            notes_notes,
        };
        use diesel::prelude::*;


        let conn = self.pool.get()
            .map_err(|_| KernelError::R2d2)?;

        let notes: Vec<domain::Note> = notes_notes::dsl::notes_notes
                .filter(notes_notes::dsl::owner_id.eq(msg.account_id))
                .filter(notes_notes::dsl::deleted_at.is_null())
                .filter(notes_notes::dsl::archived_at.is_null())
                .filter(notes_notes::dsl::removed_at.is_not_null())
                .load(&conn)?;

        return Ok(notes);
    }
}
