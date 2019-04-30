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
};


#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct FindFiles {
    pub s3_bucket: String,
    pub s3_region: String,
    pub account_id: uuid::Uuid,
}

impl Message for FindFiles {
    type Result = Result<Vec<FileResponse>, KernelError>;
}

impl Handler<FindFiles> for DbActor {
    type Result = Result<Vec<FileResponse>, KernelError>;

    fn handle(&mut self, msg: FindFiles, _: &mut Self::Context) -> Self::Result {
        use kernel::db::schema::{
            drive_files,
        };
        use diesel::prelude::*;
        use diesel::pg::expression::dsl::any;

        let conn = self.pool.get()
            .map_err(|_| KernelError::R2d2)?;


        let files: Vec<File> = drive_files::dsl::drive_files
            .filter(drive_files::dsl::owner_id.eq(msg.account_id))
            .filter(drive_files::dsl::deleted_at.is_null())
            .filter(drive_files::dsl::trashed_at.is_null())
            .filter(drive_files::dsl::type_.eq(any(vec!["audio/mpeg", "audio/mp3"])))
            .load(&conn)?;

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

        return Ok(files);
    }
}
