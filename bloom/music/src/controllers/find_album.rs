use actix::{Message, Handler};
use serde::{Serialize, Deserialize};
use kernel::{
    db::DbActor,
    KernelError,
};
use rusoto_s3::{
    GetObjectRequest,
    util::PreSignedRequest,
};
use rusoto_core::{Region};
use rusoto_credential::{EnvironmentProvider, ProvideAwsCredentials};
use drive::domain::File;
use std::str::FromStr;
use futures::Future;
use crate::{
    api::v1::models::FileResponse,
    domain::album,
};


#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct FindAlbum {
    pub album_id: uuid::Uuid,
    pub account_id: uuid::Uuid,
    pub s3_bucket: String,
    pub s3_region: String,
}

impl Message for FindAlbum {
    type Result = Result<(album::Album, Vec<FileResponse>), KernelError>;
}

impl Handler<FindAlbum> for DbActor {
    type Result = Result<(album::Album, Vec<FileResponse>), KernelError>;

    fn handle(&mut self, msg: FindAlbum, _: &mut Self::Context) -> Self::Result {
        use kernel::db::schema::{
            gallery_albums,
            gallery_albums_files,
            drive_files,
        };
        use diesel::{
            // sql_query,
            // pg::types::sql_types,
            prelude::*,
            pg::expression::dsl::any,
        };


        let conn = self.pool.get()
            .map_err(|_| KernelError::R2d2)?;

        // join avec gallery_albums_files
        // il nous faut l'album + les files
        let album: album::Album = gallery_albums::dsl::gallery_albums
            .filter(gallery_albums::dsl::owner_id.eq(msg.account_id))
            .filter(gallery_albums::dsl::deleted_at.is_null())
            .filter(gallery_albums::dsl::id.eq(msg.album_id))
            .first(&conn)?;

        let files: Vec<(File, album::AlbumFile)> = drive_files::dsl::drive_files
            .inner_join(gallery_albums_files::table)
            .filter(gallery_albums_files::dsl::album_id.eq(msg.album_id))
            .filter(drive_files::dsl::deleted_at.is_null())
            .filter(drive_files::dsl::trashed_at.is_null())
            .load(&conn)?;
        let files: Vec<File> = files.into_iter().map(|file| file.0).collect();

        // let files: Vec<File> = sql_query("SELECT * FROM drive_files AS file
        //     INNER JOIN gallery_albums_files AS file ON file.id = file.file_id
        //     WHERE file.album_id = $1 AND file.deleted_at IS NULL AND file.trashed_at IS NULL")
        //     .bind::<sql_types::Uuid, _>(msg.album_id)
        //     .load(&conn)?;

        let region = Region::from_str(&msg.s3_region).expect("AWS region not valid");
        let credentials = EnvironmentProvider::default()
            .credentials()
            .wait()
            .expect("error getting default credentials");

        let files: Vec<FileResponse> = files.into_iter().map(|file| {
            let key = format!("drive/{}/{}", file.owner_id, file.id);
            let req = GetObjectRequest {
                bucket: msg.s3_bucket.clone(),
                key: key,
                // response_content_disposition: Some(format!(r#"attachment; filename="{}""#, &aggregate.name)),
                response_content_type: Some(file.type_.clone()),
                ..Default::default()
            };
            // TODO: handle error
            let presigned_url = req.get_presigned_url(&region, &credentials, &Default::default());
            return FileResponse{
                id: file.id,
                name: file.name,
                parent_id: file.parent_id.unwrap(),
                url: presigned_url,
            };
        }).collect();

        return Ok((album, files));
    }
}
