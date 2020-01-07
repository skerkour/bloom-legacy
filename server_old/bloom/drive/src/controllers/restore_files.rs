use crate::{domain, domain::file};
use actix::{Handler, Message};
use kernel::{db::DbActor, KernelError};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RestoreFiles {
    pub files: Vec<uuid::Uuid>,
    pub owner_id: uuid::Uuid,
    pub request_id: uuid::Uuid,
    pub session_id: uuid::Uuid,
}

impl Message for RestoreFiles {
    type Result = Result<(), KernelError>;
}

// TODO: recursively restore
impl Handler<RestoreFiles> for DbActor {
    type Result = Result<(), KernelError>;

    fn handle(&mut self, msg: RestoreFiles, _: &mut Self::Context) -> Self::Result {
        use diesel::prelude::*;
        use kernel::db::schema::drive_files;

        let conn = self.pool.get().map_err(|_| KernelError::R2d2)?;

        return Ok(conn.transaction::<_, KernelError, _>(|| {
            for file_id in msg.files.into_iter() {
                let file_to_restore: domain::File = drive_files::dsl::drive_files
                    .filter(drive_files::dsl::id.eq(file_id))
                    .filter(drive_files::dsl::owner_id.eq(msg.owner_id))
                    .first(&conn)?;

                let restore_cmd = file::Restore {};
                let (file_to_restore, _) =
                    eventsourcing::execute(&conn, file_to_restore, &restore_cmd)?;
                diesel::update(&file_to_restore)
                    .set(&file_to_restore)
                    .execute(&conn)?;
            }

            return Ok(());
        })?);
    }
}
