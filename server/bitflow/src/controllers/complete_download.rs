use crate::domain;
use actix::{Handler, Message};
use kernel::{db::DbActor, KernelError};

#[derive(Clone)]
pub struct CompleteDownload {
    // pub actor_id: uuid::Uuid,
    // pub session_id: uuid::Uuid,
    pub s3_bucket: String,
    pub s3_client: rusoto_s3::S3Client,
    pub download_id: uuid::Uuid,
    pub complete_data: domain::download::CompleteData,
    pub request_id: uuid::Uuid,
}

impl Message for CompleteDownload {
    type Result = Result<domain::Download, KernelError>;
}

impl Handler<CompleteDownload> for DbActor {
    type Result = Result<domain::Download, KernelError>;

    fn handle(&mut self, msg: CompleteDownload, _: &mut Self::Context) -> Self::Result {
        use diesel::prelude::*;
        use kernel::db::schema::{bitflow_downloads, bitflow_profiles, drive_profiles};

        let conn = self.pool.get().map_err(|_| KernelError::R2d2)?;

        return Ok(conn.transaction::<_, KernelError, _>(|| {
            let download: domain::Download = bitflow_downloads::dsl::bitflow_downloads
                .filter(bitflow_downloads::dsl::id.eq(msg.download_id))
                .first(&conn)?;

            let profile: domain::Profile = bitflow_profiles::dsl::bitflow_profiles
                .filter(bitflow_profiles::dsl::account_id.eq(download.owner_id))
                .first(&conn)?;

            let complete_cmd = domain::download::Complete {
                s3_bucket: msg.s3_bucket,
                s3_client: msg.s3_client,
                profile,
                data: msg.complete_data.clone(),
            };

            let (download, _) = eventsourcing::execute(&conn, download, &complete_cmd)?;
            diesel::update(&download).set(&download).execute(&conn)?;

            let total_size = msg.complete_data.files.iter().fold(0i64, |acc, x| {
                return acc + x.size as i64;
            });

            // update drive profile: Add all uploaded data size
            let drive_profile: drive::domain::Profile = drive_profiles::dsl::drive_profiles
                .filter(drive_profiles::dsl::account_id.eq(download.owner_id))
                .first(&conn)?;

            let space_cmd = drive::domain::profile::UpdateUsedSpace { space: total_size };
            let (drive_profile, _) = eventsourcing::execute(&conn, drive_profile, &space_cmd)?;

            diesel::update(&drive_profile)
                .set(&drive_profile)
                .execute(&conn)?;

            return Ok(download);
        })?);
    }
}
