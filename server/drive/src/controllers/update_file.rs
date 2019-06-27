use crate::domain::file;
use actix::{Handler, Message};
use kernel::{db::DbActor, events::EventMetadata, KernelError};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UpdateFile {
    pub name: Option<String>,
    pub file_id: uuid::Uuid,
    pub actor_id: uuid::Uuid,
    pub session_id: uuid::Uuid,
    pub request_id: uuid::Uuid,
}

impl Message for UpdateFile {
    type Result = Result<file::File, KernelError>;
}

impl Handler<UpdateFile> for DbActor {
    type Result = Result<file::File, KernelError>;

    fn handle(&mut self, msg: UpdateFile, _: &mut Self::Context) -> Self::Result {
        use diesel::prelude::*;
        use kernel::db::schema::{drive_files, drive_files_events};

        let conn = self.pool.get().map_err(|_| KernelError::R2d2)?;

        return Ok(conn.transaction::<_, KernelError, _>(|| {
            let metadata = EventMetadata {
                actor_id: Some(msg.actor_id),
                request_id: Some(msg.request_id),
                session_id: Some(msg.session_id),
            };

            let file_to_update: file::File = drive_files::dsl::drive_files
                .filter(drive_files::dsl::id.eq(msg.file_id))
                .filter(drive_files::dsl::owner_id.eq(msg.actor_id))
                .filter(drive_files::dsl::deleted_at.is_null())
                .for_update()
                .first(&conn)?;

            // name
            let file_to_update = match &msg.name {
                Some(name) if name != &file_to_update.name => {
                    let rename_cmd = file::Rename {
                        name: name.clone(),
                        metadata: metadata.clone(),
                    };

                    let (file_to_update, event, _) =
                        eventsourcing::execute(&conn, file_to_update, &rename_cmd)?;

                    // update note
                    diesel::update(&file_to_update)
                        .set(&file_to_update)
                        .execute(&conn)?;
                    diesel::insert_into(drive_files_events::dsl::drive_files_events)
                        .values(&event)
                        .execute(&conn)?;
                    file_to_update
                }
                _ => file_to_update,
            };

            return Ok(file_to_update);
        })?);
    }
}
