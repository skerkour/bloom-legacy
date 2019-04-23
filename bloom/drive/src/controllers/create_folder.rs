use actix::{Message, Handler};
use serde::{Serialize, Deserialize};
use kernel::{
    db::DbActor,
    KernelError,
    events::EventMetadata,
};
use crate::{
    domain,
    FOLDER_TYPE,
    domain::file,
};


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
        use kernel::db::schema::{
            drive_files,
            drive_files_events,
        };
        use diesel::prelude::*;

        let conn = self.pool.get()
            .map_err(|_| KernelError::R2d2)?;

        return Ok(conn.transaction::<_, KernelError, _>(|| {

            let parent: domain::File = drive_files::dsl::drive_files
                .filter(drive_files::dsl::id.eq(msg.parent_id))
                .filter(drive_files::dsl::owner_id.eq(msg.owner_id))
                .filter(drive_files::dsl::deleted_at.is_null())
                .filter(drive_files::dsl::trashed_at.is_null())
                .first(&conn)?;

            let metadata = EventMetadata{
                actor_id: Some(msg.owner_id),
                request_id: Some(msg.request_id),
                session_id: Some(msg.session_id),
            };

            let create_cmd = file::Create{
                name: msg.name,
                type_: FOLDER_TYPE.to_string(),
                size: 0,
                parent_id: Some(parent.id),
                owner_id: msg.owner_id,
                metadata: metadata.clone(),
            };
            let (new_folder, event, _) = eventsourcing::execute(&conn, file::File::new(), &create_cmd)?;
            diesel::insert_into(drive_files::dsl::drive_files)
                .values(&new_folder)
                .execute(&conn)?;
            diesel::insert_into(drive_files_events::dsl::drive_files_events)
                .values(&event)
                .execute(&conn)?;

            return Ok(new_folder);
        })?);
    }
}
