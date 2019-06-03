use actix::{Message, Handler};
use kernel::{
    KernelError,
    events::EventMetadata,
    db::DbActor
};
use std::fs;
use crate::domain::{
    Upload,
    upload,
    file,
    File,
    profile,
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
            drive_profiles,
            drive_profiles_events,
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

            let drive_profile: profile::Profile = drive_profiles::dsl::drive_profiles
                .filter(drive_profiles::dsl::account_id.eq(msg.account_id))
                .filter(drive_profiles::dsl::deleted_at.is_null())
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
                s3_client: msg.s3_client.clone(),
                file_path: msg.file_path.clone(),
                metadata: metadata.clone(),
            };
            let (upload_to_update, event, _) = eventsourcing::execute(
                &conn, upload_to_update, &complete_cmd)?;

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
                metadata: metadata.clone(),
            };
            let (uploaded_file, event, _) = eventsourcing::execute(&conn, File::new(), &upload_cmd)?;

            diesel::insert_into(drive_files::dsl::drive_files)
                .values(&uploaded_file)
                .execute(&conn)?;
            diesel::insert_into(drive_files_events::dsl::drive_files_events)
                .values(&event)
                .execute(&conn)?;

            // update profile
            let space_cmd = profile::UpdateUsedSpace{
                space: upload_to_update.size,
                metadata: metadata.clone(),
            };
            let (drive_profile, event, _) = eventsourcing::execute(&conn, drive_profile, &space_cmd)?;

            diesel::update(&drive_profile)
                .set(&drive_profile)
                .execute(&conn)?;
            diesel::insert_into(drive_profiles_events::dsl::drive_profiles_events)
                .values(&event)
                .execute(&conn)?;

            // remove upload directory
            fs::remove_dir_all(&msg.directory)?;

            return Ok(uploaded_file);
        })?);
    }
}
