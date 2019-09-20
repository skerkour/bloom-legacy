use crate::{domain, domain::download};
use diesel::{
    r2d2::{ConnectionManager, PooledConnection},
    PgConnection,
};
use drive::domain::file;
use eventsourcing::{Event, EventTs};
use kernel::KernelError;
use rusoto_s3::{CopyObjectRequest, S3};

#[derive(Clone)]
pub struct Complete {
    pub data: download::CompleteData,
    pub s3_bucket: String,
    pub s3_client: rusoto_s3::S3Client,
    pub profile: domain::Profile,
}

impl eventsourcing::Command for Complete {
    type Aggregate = download::Download;
    type Event = Completed;
    type Context = PooledConnection<ConnectionManager<PgConnection>>;
    type Error = KernelError;

    fn validate(
        &self,
        _ctx: &Self::Context,
        aggregate: &Self::Aggregate,
    ) -> Result<(), Self::Error> {
        if aggregate.removed_at.is_some() {
            return Err(KernelError::Validation(
                "Download has been removed".to_string(),
            ));
        }

        if aggregate.status != download::DownloadStatus::Downloading {
            return Err(KernelError::Validation(
                "Download is not in the Downloading state".to_string(),
            ));
        }

        return Ok(());
    }

    fn build_event(
        &self,
        ctx: &Self::Context,
        aggregate: &Self::Aggregate,
    ) -> Result<Self::Event, Self::Error> {
        use diesel::prelude::*;
        use kernel::db::schema::drive_files;

        let files = self
            .data
            .files
            .iter()
            .map(|file| {
                // create file in drive
                let upload_cmd = file::Upload {
                    id: uuid::Uuid::new_v4(),
                    name: file.name.clone(),
                    parent_id: Some(self.profile.download_folder_id),
                    size: file.size as i64,
                    type_: file.type_.clone(),
                    owner_id: aggregate.owner_id,
                };
                let (uploaded_file, _) =
                    eventsourcing::execute(ctx, file::File::new(), &upload_cmd)?;
                diesel::insert_into(drive_files::dsl::drive_files)
                    .values(&uploaded_file)
                    .execute(ctx)?;

                // copy s3 file
                // TODO: delete bitflow file
                let from = format!(
                    "{}/bitflow/{}/{}",
                    &self.s3_bucket, aggregate.id, file.bitflow_id
                );
                let to = format!("drive/{}/{}", aggregate.owner_id, uploaded_file.id);
                let req = CopyObjectRequest {
                    bucket: self.s3_bucket.clone(),
                    key: to,
                    copy_source: from,
                    content_type: Some(uploaded_file.type_),
                    ..Default::default()
                };
                // TODO: handle error
                let _ = self
                    .s3_client
                    .copy_object(req)
                    .sync()
                    .expect("Couldn't copy object");
                return Ok(uploaded_file.id);
            })
            .collect::<Result<Vec<uuid::Uuid>, KernelError>>()?;

        return Ok(Completed {
            timestamp: chrono::Utc::now(),
            files,
        });
    }
}

// Event
#[derive(Clone, Debug, EventTs)]
pub struct Completed {
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub files: Vec<uuid::Uuid>,
}

impl Event for Completed {
    type Aggregate = download::Download;

    fn apply(&self, aggregate: Self::Aggregate) -> Self::Aggregate {
        return Self::Aggregate {
            status: download::DownloadStatus::Success,
            progress: 100,
            ..aggregate
        };
    }
}
