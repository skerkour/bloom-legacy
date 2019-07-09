use crate::{domain, domain::file};
use actix::{Handler, Message};
use kernel::{db::DbActor, KernelError};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TrashFiles {
    pub files: Vec<uuid::Uuid>,
    pub owner_id: uuid::Uuid,
    pub request_id: uuid::Uuid,
    pub session_id: uuid::Uuid,
}

impl Message for TrashFiles {
    type Result = Result<(), KernelError>;
}

// TODO: recursively trash
impl Handler<TrashFiles> for DbActor {
    type Result = Result<(), KernelError>;

    fn handle(&mut self, msg: TrashFiles, _: &mut Self::Context) -> Self::Result {
        use diesel::prelude::*;
        use kernel::db::schema::drive_files;

        let conn = self.pool.get().map_err(|_| KernelError::R2d2)?;

        return Ok(conn.transaction::<_, KernelError, _>(|| {
            for file_id in msg.files.into_iter() {
                let file_to_trash: domain::File = drive_files::dsl::drive_files
                    .filter(drive_files::dsl::id.eq(file_id))
                    .filter(drive_files::dsl::owner_id.eq(msg.owner_id))
                    .first(&conn)?;

                let trash_cmd = file::Trash {
                    explicitly_trashed: true,
                };
                let (file_to_trash, _) = eventsourcing::execute(&conn, file_to_trash, &trash_cmd)?;
                diesel::update(&file_to_trash)
                    .set(&file_to_trash)
                    .execute(&conn)?;
            }

            return Ok(());
        })?);
    }
}
