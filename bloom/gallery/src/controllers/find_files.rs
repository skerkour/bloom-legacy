use actix::{Message, Handler};
use serde::{Serialize, Deserialize};
use kernel::{
    db::DbActor,
    KernelError,
};
use crate::domain::album;


#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct FindFiles {
    pub account_id: uuid::Uuid,
}

impl Message for FindFiles {
    type Result = Result<Vec<album::File>, KernelError>;
}

impl Handler<FindFiles> for DbActor {
    type Result = Result<Vec<album::File>, KernelError>;

    fn handle(&mut self, msg: FindFiles, _: &mut Self::Context) -> Self::Result {
        use kernel::db::schema::{
            drive_files,
        };
        use diesel::prelude::*;

        let conn = self.pool.get()
            .map_err(|_| KernelError::R2d2)?;

        let files = drive_files::dsl::drive_files
            .filter(drive_files::dsl::owner_id.eq(msg.account_id))
            .filter(drive_files::dsl::deleted_at.is_null())
            .filter(drive_files::dsl::trashed_at.is_null())
            // .filter() // type
            .load(&conn)?;

        return Ok(files);
    }
}
