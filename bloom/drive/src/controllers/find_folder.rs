use actix::{Message, Handler};
use serde::{Serialize, Deserialize};
use kernel::{
    db::DbActor,
    KernelError,
};
use diesel::{
    sql_query,
    pg::types::sql_types,
};
use crate::{
    domain,
    FOLDER_TYPE,
    BLOOM_ROOT_NAME,
    domain::file::FolderPath,
};


#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct FindFolder {
    pub id: Option<uuid::Uuid>,
    pub owner_id: uuid::Uuid,
}

impl Message for FindFolder {
    type Result = Result<(Vec<FolderPath>, domain::File, Vec<domain::File>), KernelError>;
}

impl Handler<FindFolder> for DbActor {
    type Result = Result<(Vec<FolderPath>, domain::File, Vec<domain::File>), KernelError>;

    fn handle(&mut self, msg: FindFolder, _: &mut Self::Context) -> Self::Result {
        use kernel::db::schema::{
            drive_files,
        };
        use diesel::prelude::*;

        let conn = self.pool.get()
            .map_err(|_| KernelError::R2d2)?;

        let folder: domain::File = match msg.id {
            Some(id) => {
                drive_files::dsl::drive_files
                    .filter(drive_files::dsl::id.eq(id))
                    .filter(drive_files::dsl::owner_id.eq(msg.owner_id))
                    .filter(drive_files::dsl::type_.eq(FOLDER_TYPE))
                    .filter(drive_files::dsl::deleted_at.is_null())
                    .filter(drive_files::dsl::removed_at.is_null())
                    .first(&conn)?
            },
            // retrieve root folder
            None => {
                drive_files::dsl::drive_files
                    .filter(drive_files::dsl::name.eq(BLOOM_ROOT_NAME))
                    .filter(drive_files::dsl::owner_id.eq(msg.owner_id))
                    .filter(drive_files::dsl::type_.eq(FOLDER_TYPE))
                    .filter(drive_files::dsl::deleted_at.is_null())
                    .filter(drive_files::dsl::removed_at.is_null())
                    .first(&conn)?
            }
        };

        // find children
        let children: Vec<domain::File> = drive_files::dsl::drive_files
            .filter(drive_files::dsl::parent_id.eq(folder.id))
            .filter(drive_files::dsl::deleted_at.is_null())
            .filter(drive_files::dsl::removed_at.is_null())
            .load(&conn)?;

        let path: Vec<FolderPath> = sql_query(include_str!("../../sql_requests/file_path.sql"))
            .bind::<sql_types::Uuid, _>(folder.id)
            .load(&conn)?;

        return Ok((path, folder, children));
    }
}
