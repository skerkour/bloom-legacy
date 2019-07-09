use crate::domain;
use actix::{Handler, Message};
use kernel::{db::DbActor, KernelError};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct FindTrashed {
    pub owner_id: uuid::Uuid,
}

impl Message for FindTrashed {
    type Result = Result<Vec<domain::File>, KernelError>;
}

impl Handler<FindTrashed> for DbActor {
    type Result = Result<Vec<domain::File>, KernelError>;

    fn handle(&mut self, msg: FindTrashed, _: &mut Self::Context) -> Self::Result {
        use diesel::prelude::*;
        use kernel::db::schema::drive_files;

        let conn = self.pool.get().map_err(|_| KernelError::R2d2)?;

        // find children
        let trashed: Vec<domain::File> = drive_files::dsl::drive_files
            .filter(drive_files::dsl::owner_id.eq(msg.owner_id))
            .filter(drive_files::dsl::trashed_at.is_not_null())
            .filter(drive_files::dsl::explicitly_trashed.eq(true))
            .load(&conn)?;

        return Ok(trashed);
    }
}
