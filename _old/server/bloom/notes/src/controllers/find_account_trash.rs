use crate::domain;
use actix::{Handler, Message};
use kernel::{db::DbActor, error::KernelError};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct FindAccountTrash {
    pub account_id: uuid::Uuid,
}

impl Message for FindAccountTrash {
    type Result = Result<Vec<domain::Note>, KernelError>;
}

impl Handler<FindAccountTrash> for DbActor {
    type Result = Result<Vec<domain::Note>, KernelError>;

    fn handle(&mut self, msg: FindAccountTrash, _: &mut Self::Context) -> Self::Result {
        use diesel::prelude::*;
        use kernel::db::schema::notes_notes;

        let conn = self.pool.get().map_err(|_| KernelError::R2d2)?;

        let notes: Vec<domain::Note> = notes_notes::dsl::notes_notes
            .filter(notes_notes::dsl::owner_id.eq(msg.account_id))
            .filter(notes_notes::dsl::removed_at.is_not_null())
            .order_by(notes_notes::dsl::updated_at.desc())
            .load(&conn)?;

        return Ok(notes);
    }
}
