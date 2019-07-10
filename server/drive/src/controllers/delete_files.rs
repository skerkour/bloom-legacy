use crate::{domain, domain::file, domain::profile};
use actix::{Handler, Message};
use diesel::{
    pg::types::sql_types,
    r2d2::{ConnectionManager, PooledConnection},
    sql_query, PgConnection,
};
use kernel::{db::DbActor, KernelError};

#[derive(Clone)]
pub struct DeleteFiles {
    pub files: Vec<uuid::Uuid>,
    pub s3_bucket: String,
    pub s3_client: rusoto_s3::S3Client,
    pub owner_id: uuid::Uuid,
    pub request_id: uuid::Uuid,
    pub session_id: uuid::Uuid,
}

impl Message for DeleteFiles {
    type Result = Result<i64, KernelError>;
}

// TODO: recursively delete
impl Handler<DeleteFiles> for DbActor {
    type Result = Result<i64, KernelError>;

    fn handle(&mut self, msg: DeleteFiles, _: &mut Self::Context) -> Self::Result {
        use diesel::prelude::*;
        use kernel::db::schema::drive_files;

        let conn = self.pool.get().map_err(|_| KernelError::R2d2)?;

        return Ok(conn.transaction::<_, KernelError, _>(|| {
            let mut space_freed = 0i64;

            for file_id in msg.files.into_iter() {
                let file_to_delete: domain::File = drive_files::dsl::drive_files
                    .filter(drive_files::dsl::id.eq(file_id))
                    .filter(drive_files::dsl::owner_id.eq(msg.owner_id))
                    .first(&conn)?;

                if file_to_delete.name == crate::BLOOM_ROOT_NAME {
                    return Err(KernelError::Validation(
                        "You can not delete Home".to_string(),
                    ));
                }

                space_freed += delete_file_with_children(
                    file_to_delete.clone(),
                    msg.s3_client.clone(),
                    msg.s3_bucket.clone(),
                    &conn,
                )?;
            }

            return Ok(space_freed);
        })?);
    }
}

pub fn delete_file_with_children(
    file: domain::File,
    s3_client: rusoto_s3::S3Client,
    s3_bucket: String,
    conn: &PooledConnection<ConnectionManager<PgConnection>>,
) -> Result<i64, KernelError> {
    use diesel::prelude::*;
    use kernel::db::schema::{drive_files, drive_profiles};

    let mut space_freed = 0i64;

    if file.type_ == crate::FOLDER_TYPE {
        // find children
        let folder_children: Vec<file::FolderChild> =
            sql_query(include_str!("../../sql_requests/folder_children.sql"))
                .bind::<sql_types::Uuid, _>(file.id)
                .load(conn)?;

        // delete children
        for child in folder_children.iter() {
            let file_to_delete: domain::File = drive_files::dsl::drive_files
                .filter(drive_files::dsl::id.eq(child.id))
                .filter(drive_files::dsl::owner_id.eq(file.owner_id))
                .first(conn)?;

            let delete_cmd = file::Delete {
                s3_client: s3_client.clone(),
                s3_bucket: s3_bucket.clone(),
            };
            let (file_to_delete, _) = eventsourcing::execute(conn, file_to_delete, &delete_cmd)?;

            // recursively
            space_freed += delete_file_with_children(
                file_to_delete,
                s3_client.clone(),
                s3_bucket.clone(),
                conn,
            )?;
        }
    } else {
        // update profile
        let space_cmd = profile::UpdateUsedSpace { space: -file.size };
        space_freed += file.size;
        let drive_profile: profile::Profile =
            drive_profiles::dsl::drive_profiles // TODO: ULTRA UGLY...
                .filter(drive_profiles::dsl::account_id.eq(file.owner_id))
                .for_update()
                .first(conn)?;
        let (drive_profile, _) = eventsourcing::execute(conn, drive_profile, &space_cmd)?;

        diesel::update(&drive_profile)
            .set(&drive_profile)
            .execute(conn)?;
    }

    // delete file, all children will be delete thanks to PostgreSQL cascading
    let delete_cmd = file::Delete {
        s3_client: s3_client.clone(),
        s3_bucket: s3_bucket.clone(),
    };
    let (file, _) = eventsourcing::execute(conn, file, &delete_cmd)?;

    diesel::delete(&file).execute(conn)?;

    return Ok(space_freed);
}
