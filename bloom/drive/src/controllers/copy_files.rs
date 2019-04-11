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
    BLOOM_ROOT_NAME,
    domain::file,
};


#[derive(Clone)]
pub struct CopyFiles {
    pub to: uuid::Uuid,
    pub files: Vec<uuid::Uuid>,
    pub owner_id: uuid::Uuid,
    pub request_id: uuid::Uuid,
    pub session_id: uuid::Uuid,
    pub s3_bucket: String,
    pub s3_client: rusoto_s3::S3Client,
}

impl Message for CopyFiles {
    type Result = Result<(), KernelError>;
}

impl Handler<CopyFiles> for DbActor {
    type Result = Result<(), KernelError>;

    fn handle(&mut self, msg: CopyFiles, _: &mut Self::Context) -> Self::Result {
        use kernel::db::schema::{
            drive_files,
            drive_files_events,
        };
        use diesel::prelude::*;

        let conn = self.pool.get()
            .map_err(|_| KernelError::R2d2)?;

        return Ok(conn.transaction::<_, KernelError, _>(|| {

            let metadata = EventMetadata{
                actor_id: Some(msg.owner_id),
                request_id: Some(msg.request_id),
                session_id: Some(msg.session_id),
            };

            for file_id in msg.files.into_iter() {

                let file_to_copy: domain::File = drive_files::dsl::drive_files
                    .filter(drive_files::dsl::id.eq(file_id))
                    .filter(drive_files::dsl::owner_id.eq(msg.owner_id))
                    .filter(drive_files::dsl::deleted_at.is_null())
                    .filter(drive_files::dsl::trashed_at.is_null())
                    .first(&conn)?;

                // create new file
                let create_cmd = file::Create{
                    name: file_to_copy.name.clone(),
                    type_: file_to_copy.type_.clone(),
                    size: file_to_copy.size,
                    parent_id: Some(msg.to),
                    owner_id: file_to_copy.owner_id,
                    metadata: metadata.clone(),
                };
                let (new_file, event, _) = eventsourcing::execute(&conn, file::File::new(), &create_cmd)?;
                diesel::insert_into(drive_files::dsl::drive_files)
                    .values(&new_file)
                    .execute(&conn)?;
                diesel::insert_into(drive_files_events::dsl::drive_files_events)
                    .values(&event)
                    .execute(&conn)?;

                // copy file
                let copy_cmd = file::Copy_{
                    to: msg.to,
                    new_file: new_file.id,
                    s3_client: msg.s3_client.clone(),
                    s3_bucket: msg.s3_bucket.clone(),
                    metadata: metadata.clone(),
                };
                let (file_to_copy, event, _) = eventsourcing::execute(&conn, file_to_copy, &copy_cmd)?;
                diesel::update(&file_to_copy)
                    .set(&file_to_copy)
                    .execute(&conn)?;
                diesel::insert_into(drive_files_events::dsl::drive_files_events)
                    .values(&event)
                    .execute(&conn)?;
            }

            return Ok(());
        })?);
    }
}
