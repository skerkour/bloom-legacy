use crate::{domain, domain::file, domain::profile};
use actix::{Handler, Message};
use diesel::{pg::types::sql_types, sql_query};
use kernel::{db::DbActor, events::EventMetadata, KernelError};
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
        use kernel::db::schema::{
            drive_files, drive_files_events, drive_profiles, drive_profiles_events,
        };

        let conn = self.pool.get().map_err(|_| KernelError::R2d2)?;

        return Ok(conn.transaction::<_, KernelError, _>(|| {
            let metadata = EventMetadata {
                actor_id: Some(msg.owner_id),
                request_id: Some(msg.request_id),
                session_id: Some(msg.session_id),
            };

            let mut space_freed = 0i64;

            for file_id in msg.files.into_iter() {
                let file_to_delete: domain::File = drive_files::dsl::drive_files
                    .filter(drive_files::dsl::id.eq(file_id))
                    .filter(drive_files::dsl::owner_id.eq(msg.owner_id))
                    .filter(drive_files::dsl::deleted_at.is_null())
                    .first(&conn)?;

                let delete_cmd = file::Delete {
                    metadata: metadata.clone(),
                };
                let (file_to_delete, event, _) =
                    eventsourcing::execute(&conn, file_to_delete, &delete_cmd)?;
                diesel::update(&file_to_delete)
                    .set(&file_to_delete)
                    .execute(&conn)?;
                diesel::insert_into(drive_files_events::dsl::drive_files_events)
                    .values(&event)
                    .execute(&conn)?;

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
                            .filter(drive_files::dsl::deleted_at.is_null())
                            .first(&conn)?;

                        let delete_cmd = file::Delete {
                            metadata: metadata.clone(),
                        };
                        let (file_to_delete, event, _) =
                            eventsourcing::execute(&conn, file_to_delete, &delete_cmd)?;
                        diesel::update(&file_to_delete)
                            .set(&file_to_delete)
                            .execute(&conn)?;
                        diesel::insert_into(drive_files_events::dsl::drive_files_events)
                            .values(&event)
                            .execute(&conn)?;

                        if file_to_delete.type_ != crate::FOLDER_TYPE {
                            // update profile
                            let space_cmd = profile::UpdateUsedSpace {
                                space: -file_to_delete.size,
                                metadata: metadata.clone(),
                            };
                            space_freed += file_to_delete.size;
                            let drive_profile: profile::Profile = drive_profiles::dsl::drive_profiles // TODO: ULTRA UGLY...
                                .filter(drive_profiles::dsl::account_id.eq(msg.owner_id))
                                .filter(drive_profiles::dsl::deleted_at.is_null())
                                .for_update()
                                .first(&conn)?;
                            let (drive_profile, event, _) =
                                eventsourcing::execute(&conn, drive_profile, &space_cmd)?;

                            diesel::update(&drive_profile)
                                .set(&drive_profile)
                                .execute(&conn)?;
                            diesel::insert_into(drive_profiles_events::dsl::drive_profiles_events)
                                .values(&event)
                                .execute(&conn)?;
                        }
                    }
                }
                if file_to_delete.type_ != crate::FOLDER_TYPE {
                    // update profile
                    let drive_profile: profile::Profile = drive_profiles::dsl::drive_profiles // TODO: ULTRA UGLY...
                        .filter(drive_profiles::dsl::account_id.eq(msg.owner_id))
                        .filter(drive_profiles::dsl::deleted_at.is_null())
                        .for_update()
                        .first(&conn)?;
                    let space_cmd = profile::UpdateUsedSpace {
                        space: -file_to_delete.size,
                        metadata: metadata.clone(),
                    };
                    space_freed += file_to_delete.size;
                    let (drive_profile, event, _) =
                        eventsourcing::execute(&conn, drive_profile, &space_cmd)?;

                    diesel::update(&drive_profile)
                        .set(&drive_profile)
                        .execute(&conn)?;
                    diesel::insert_into(drive_profiles_events::dsl::drive_profiles_events)
                        .values(&event)
                        .execute(&conn)?;
                }
            }

            return Ok(space_freed);
        })?);
    }
}
