use crate::{domain, domain::file};
use actix::{Handler, Message};
use kernel::{db::DbActor, events::EventMetadata, KernelError};
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
        use kernel::db::schema::{drive_files, drive_files_events};

        let conn = self.pool.get().map_err(|_| KernelError::R2d2)?;

        return Ok(conn.transaction::<_, KernelError, _>(|| {
            let metadata = EventMetadata {
                actor_id: Some(msg.owner_id),
                request_id: Some(msg.request_id),
                session_id: Some(msg.session_id),
            };

            for file_id in msg.files.into_iter() {
                let file_to_trash: domain::File = drive_files::dsl::drive_files
                    .filter(drive_files::dsl::id.eq(file_id))
                    .filter(drive_files::dsl::owner_id.eq(msg.owner_id))
                    .filter(drive_files::dsl::deleted_at.is_null())
                    .first(&conn)?;

                let trash_cmd = file::Trash {
                    explicitly_trashed: true,
                    metadata: metadata.clone(),
                };
                let (file_to_trash, event, _) =
                    eventsourcing::execute(&conn, file_to_trash, &trash_cmd)?;
                diesel::update(&file_to_trash)
                    .set(&file_to_trash)
                    .execute(&conn)?;
                diesel::insert_into(drive_files_events::dsl::drive_files_events)
                    .values(&event)
                    .execute(&conn)?;
            }

            return Ok(());
        })?);
    }
}
