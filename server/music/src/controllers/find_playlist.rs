use actix::{Message, Handler};
use serde::{Serialize, Deserialize};
use kernel::{
    db::DbActor,
    KernelError,
};
use rusoto_s3::{
    GetObjectRequest,
    util::{PreSignedRequest, PreSignedRequestOption},
};
use rusoto_core::{Region};
use rusoto_credential::{EnvironmentProvider, ProvideAwsCredentials};
use drive::domain::File;
use std::str::FromStr;
use std::time::Duration;
use futures::Future;
use crate::{
    api::v1::models::FileResponse,
    domain::playlist,
};


#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct FindPlaylist {
    pub playlist_id: uuid::Uuid,
    pub account_id: uuid::Uuid,
    pub s3_bucket: String,
    pub s3_region: String,
}

impl Message for FindPlaylist {
    type Result = Result<(playlist::Playlist, Vec<FileResponse>), KernelError>;
}

impl Handler<FindPlaylist> for DbActor {
    type Result = Result<(playlist::Playlist, Vec<FileResponse>), KernelError>;

    fn handle(&mut self, msg: FindPlaylist, _: &mut Self::Context) -> Self::Result {
        use kernel::db::schema::{
            music_playlists,
            music_playlists_files,
            drive_files,
        };
        use diesel::{
            // sql_query,
            // pg::types::sql_types,
            prelude::*,
        };


        let conn = self.pool.get()
            .map_err(|_| KernelError::R2d2)?;

        // join avec music_playlists_files
        // il nous faut l'playlist + les files
        let playlist: playlist::Playlist = music_playlists::dsl::music_playlists
            .filter(music_playlists::dsl::owner_id.eq(msg.account_id))
            .filter(music_playlists::dsl::deleted_at.is_null())
            .filter(music_playlists::dsl::id.eq(msg.playlist_id))
            .first(&conn)?;

        let files: Vec<(File, playlist::PlaylistFile)> = drive_files::dsl::drive_files
            .inner_join(music_playlists_files::table)
            .filter(music_playlists_files::dsl::playlist_id.eq(msg.playlist_id))
            .filter(drive_files::dsl::deleted_at.is_null())
            .filter(drive_files::dsl::trashed_at.is_null())
            .load(&conn)?;
        let files: Vec<File> = files.into_iter().map(|file| file.0).collect();

        // let files: Vec<File> = sql_query("SELECT * FROM drive_files AS file
        //     INNER JOIN music_playlists_files AS file ON file.id = file.file_id
        //     WHERE file.playlist_id = $1 AND file.deleted_at IS NULL AND file.trashed_at IS NULL")
        //     .bind::<sql_types::Uuid, _>(msg.playlist_id)
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
            let options = PreSignedRequestOption {
                expires_in: Duration::from_secs(3600 * 24), // 24 hours
            };
            let presigned_url = req.get_presigned_url(&region, &credentials, &options);
            return FileResponse{
                id: file.id,
                name: file.name,
                parent_id: file.parent_id.unwrap(),
                url: presigned_url,
            };
        }).collect();

        return Ok((playlist, files));
    }
}
