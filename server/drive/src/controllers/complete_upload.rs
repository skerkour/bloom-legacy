use actix::{Message, Handler};
use kernel::{
    KernelError,
    events::EventMetadata,
    db::DbActor
};
use crate::domain::{
    Upload,
    upload,
    file,
    File,
};


#[derive(Clone)]
pub struct CompleteUpload {
    pub upload_id: uuid::Uuid,
    pub s3_bucket: String,
    pub s3_client: rusoto_s3::S3Client,
    pub file_path: String,
    pub directory: String,
    pub account_id: uuid::Uuid,
    pub request_id: uuid::Uuid,
    pub session_id: uuid::Uuid,
}

impl Message for CompleteUpload {
    type Result = Result<File, KernelError>;
}

impl Handler<CompleteUpload> for DbActor {
    type Result = Result<File, KernelError>;

    fn handle(&mut self, msg: CompleteUpload, _: &mut Self::Context) -> Self::Result {
        use kernel::db::schema::{
            drive_uploads,
            drive_uploads_events,
            drive_files,
            drive_files_events,
        };
        use diesel::prelude::*;


        let conn = self.pool.get()
            .map_err(|_| KernelError::R2d2)?;

        return Ok(conn.transaction::<_, KernelError, _>(|| {
            let upload_to_update: Upload = drive_uploads::dsl::drive_uploads
                .filter(drive_uploads::dsl::id.eq(msg.upload_id))
                .filter(drive_uploads::dsl::deleted_at.is_null())
                .filter(drive_uploads::dsl::owner_id.eq(msg.account_id))
                .for_update()
                .first(&conn)?;

            let metadata = EventMetadata{
                actor_id: Some(msg.account_id),
                request_id: Some(msg.request_id),
                session_id: Some(msg.session_id),
            };

            // complete Upload
            let complete_cmd = upload::Complete{
                s3_bucket: msg.s3_bucket.clone(),
                metadata: metadata.clone(),
            };
            let (upload_to_update, event, _) = eventsourcing::execute(
                &msg.s3_client, upload_to_update, &complete_cmd)?;

            diesel::update(&upload_to_update)
                .set(&upload_to_update)
                .execute(&conn)?;
            diesel::insert_into(drive_uploads_events::dsl::drive_uploads_events)
                .values(&event)
                .execute(&conn)?;

            // create file
            let upload_cmd = file::Upload{
                id: upload_to_update.file_id,
                name: upload_to_update.file_name.clone(),
                parent_id: upload_to_update.parent_id,
                size: upload_to_update.size,
                type_: upload_to_update.type_.clone(),
                owner_id: upload_to_update.owner_id,
                metadata,
            };
            let (uploaded_file, event, _) = eventsourcing::execute(&conn, File::new(), &upload_cmd)?;

            diesel::insert_into(drive_files::dsl::drive_files)
                .values(&uploaded_file)
                .execute(&conn)?;
            diesel::insert_into(drive_files_events::dsl::drive_files_events)
                .values(&event)
                .execute(&conn)?;

            // TODO: update profile

            return Ok(uploaded_file);
        })?);
    }
}
