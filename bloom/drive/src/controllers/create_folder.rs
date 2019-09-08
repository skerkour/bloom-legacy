use crate::{domain, domain::file, FOLDER_TYPE};
use actix::{Handler, Message};
use kernel::{db::DbActor, KernelError};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateFolder {
    pub parent_id: uuid::Uuid,
    pub name: String,
    pub owner_id: uuid::Uuid,
    pub request_id: uuid::Uuid,
    pub session_id: uuid::Uuid,
}

impl Message for CreateFolder {
    type Result = Result<file::File, KernelError>;
}

impl Handler<CreateFolder> for DbActor {
    type Result = Result<file::File, KernelError>;

    fn handle(&mut self, msg: CreateFolder, _: &mut Self::Context) -> Self::Result {
        use diesel::prelude::*;
        use kernel::db::schema::drive_files;

        let conn = self.pool.get().map_err(|_| KernelError::R2d2)?;

        return Ok(conn.transaction::<_, KernelError, _>(|| {
            let parent: domain::File = drive_files::dsl::drive_files
                .filter(drive_files::dsl::id.eq(msg.parent_id))
                .filter(drive_files::dsl::owner_id.eq(msg.owner_id))
                .filter(drive_files::dsl::trashed_at.is_null())
                .first(&conn)?;

            if msg.name == crate::BLOOM_ROOT_NAME {
                return Err(KernelError::Validation(
                    "file name is not valid".to_string(),
                ));
            }

            let create_cmd = file::Create {
                name: msg.name,
                type_: FOLDER_TYPE.to_string(),
                size: 0,
                parent_id: Some(parent.id),
                owner_id: msg.owner_id,
            };
            let (new_folder, _) = eventsourcing::execute(&conn, file::File::new(), &create_cmd)?;
            diesel::insert_into(drive_files::dsl::drive_files)
                .values(&new_folder)
                .execute(&conn)?;

            return Ok(new_folder);
        })?);
    }
}
