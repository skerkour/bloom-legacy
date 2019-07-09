use crate::{domain, domain::file, domain::profile};
use actix::{Handler, Message};
use diesel::{pg::types::sql_types, sql_query};
use kernel::{db::DbActor, KernelError};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DeleteFiles {
    pub files: Vec<uuid::Uuid>,
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
        use kernel::db::schema::{drive_files, drive_profiles};

        let conn = self.pool.get().map_err(|_| KernelError::R2d2)?;

        return Ok(conn.transaction::<_, KernelError, _>(|| {
            let mut space_freed = 0i64;

            for file_id in msg.files.into_iter() {
                let file_to_delete: domain::File = drive_files::dsl::drive_files
                    .filter(drive_files::dsl::id.eq(file_id))
                    .filter(drive_files::dsl::owner_id.eq(msg.owner_id))
                    .first(&conn)?;

                let delete_cmd = file::Delete {};
                let (file_to_delete, _) =
                    eventsourcing::execute(&conn, file_to_delete, &delete_cmd)?;
                diesel::delete(&file_to_delete).execute(&conn)?;

                if file_to_delete.type_ == crate::FOLDER_TYPE {
                    // find children and delete
                    let folder_children: Vec<file::FolderChild> =
                        sql_query(include_str!("../../sql_requests/folder_children.sql"))
                            .bind::<sql_types::Uuid, _>(file_to_delete.id)
                            .load(&conn)?;

                    for child in folder_children.iter() {
                        let file_to_delete: domain::File = drive_files::dsl::drive_files
                            .filter(drive_files::dsl::id.eq(child.id))
                            .filter(drive_files::dsl::owner_id.eq(msg.owner_id))
                            .first(&conn)?;

                        let delete_cmd = file::Delete {};
                        let (file_to_delete, _) =
                            eventsourcing::execute(&conn, file_to_delete, &delete_cmd)?;
                        diesel::update(&file_to_delete)
                            .set(&file_to_delete)
                            .execute(&conn)?;

                        if file_to_delete.type_ != crate::FOLDER_TYPE {
                            // update profile
                            let space_cmd = profile::UpdateUsedSpace {
                                space: -file_to_delete.size,
                            };
                            space_freed += file_to_delete.size;
                            let drive_profile: profile::Profile = drive_profiles::dsl::drive_profiles // TODO: ULTRA UGLY...
                                .filter(drive_profiles::dsl::account_id.eq(msg.owner_id))
                                .for_update()
                                .first(&conn)?;
                            let (drive_profile, _) =
                                eventsourcing::execute(&conn, drive_profile, &space_cmd)?;

                            diesel::update(&drive_profile)
                                .set(&drive_profile)
                                .execute(&conn)?;
                        }
                    }
                }
                if file_to_delete.type_ != crate::FOLDER_TYPE {
                    // update profile
                    let drive_profile: profile::Profile = drive_profiles::dsl::drive_profiles // TODO: ULTRA UGLY...
                        .filter(drive_profiles::dsl::account_id.eq(msg.owner_id))
                        .for_update()
                        .first(&conn)?;
                    let space_cmd = profile::UpdateUsedSpace {
                        space: -file_to_delete.size,
                    };
                    space_freed += file_to_delete.size;
                    let (drive_profile, _) =
                        eventsourcing::execute(&conn, drive_profile, &space_cmd)?;

                    diesel::update(&drive_profile)
                        .set(&drive_profile)
                        .execute(&conn)?;
                }
            }

            return Ok(space_freed);
        })?);
    }
}
