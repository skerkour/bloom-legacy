use crate::domain::{file, profile, upload, File, Upload};
use actix::{Handler, Message};
use kernel::{db::DbActor, KernelError};
use std::fs;

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
        use diesel::prelude::*;
        use kernel::db::schema::{drive_files, drive_profiles, drive_uploads};

        let conn = self.pool.get().map_err(|_| KernelError::R2d2)?;

        return Ok(conn.transaction::<_, KernelError, _>(|| {
            let upload_to_update: Upload = drive_uploads::dsl::drive_uploads
                .filter(drive_uploads::dsl::id.eq(msg.upload_id))
                .filter(drive_uploads::dsl::owner_id.eq(msg.account_id))
                .for_update()
                .first(&conn)?;

            let drive_profile: profile::Profile = drive_profiles::dsl::drive_profiles
                .filter(drive_profiles::dsl::account_id.eq(msg.account_id))
                .for_update()
                .first(&conn)?;

            // complete Upload
            let complete_cmd = upload::Complete {
                s3_bucket: msg.s3_bucket.clone(),
                s3_client: msg.s3_client.clone(),
                file_path: msg.file_path.clone(),
            };
            let (upload_to_update, _) =
                eventsourcing::execute(&conn, upload_to_update, &complete_cmd)?;

            diesel::delete(&upload_to_update).execute(&conn)?;

            // create file
            let upload_cmd = file::Upload {
                id: upload_to_update.file_id,
                name: upload_to_update.file_name.clone(),
                parent_id: upload_to_update.parent_id,
                size: upload_to_update.size,
                type_: upload_to_update.type_.clone(),
                owner_id: upload_to_update.owner_id,
            };
            let (uploaded_file, _) = eventsourcing::execute(&conn, File::new(), &upload_cmd)?;

            diesel::insert_into(drive_files::dsl::drive_files)
                .values(&uploaded_file)
                .execute(&conn)?;

            // update profile
            let space_cmd = profile::UpdateUsedSpace {
                space: upload_to_update.size,
            };
            let (drive_profile, _) = eventsourcing::execute(&conn, drive_profile, &space_cmd)?;

            diesel::update(&drive_profile)
                .set(&drive_profile)
                .execute(&conn)?;

            // remove upload directory
            fs::remove_dir_all(&msg.directory)?;

            return Ok(uploaded_file);
        })?);
    }
}
