use crate::{domain, domain::download};
use diesel::{
    r2d2::{ConnectionManager, PooledConnection},
    PgConnection,
};
use drive::domain::file;
use kernel::{events::EventMetadata, KernelError};
use rusoto_s3::{CopyObjectRequest, S3};

#[derive(Clone)]
pub struct Complete {
    pub data: download::CompleteData,
    pub s3_bucket: String,
    pub s3_client: rusoto_s3::S3Client,
    pub profile: domain::Profile,
    pub metadata: EventMetadata,
}

impl eventsourcing::Command for Complete {
    type Aggregate = download::Download;
    type Event = download::Event;
    type Context = PooledConnection<ConnectionManager<PgConnection>>;
    type Error = KernelError;
    type NonStoredData = ();

    fn validate(
        &self,
        _ctx: &Self::Context,
        aggregate: &Self::Aggregate,
    ) -> Result<(), Self::Error> {
        if aggregate.deleted_at.is_some() {
            return Err(KernelError::NotFound("Download not found".to_string()));
        }

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
    ) -> Result<(Self::Event, Self::NonStoredData), Self::Error> {
        use diesel::prelude::*;
        use kernel::db::schema::{drive_files, drive_files_events};

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
                    metadata: self.metadata.clone(),
                };
                let (uploaded_file, event, _) =
                    eventsourcing::execute(ctx, file::File::new(), &upload_cmd)?;
                diesel::insert_into(drive_files::dsl::drive_files)
                    .values(&uploaded_file)
                    .execute(ctx)?;
                diesel::insert_into(drive_files_events::dsl::drive_files_events)
                    .values(&event)
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

        let data = download::EventData::CompletedV1(download::CompletedV1 { files });
        return Ok((
            download::Event {
                id: uuid::Uuid::new_v4(),
                timestamp: chrono::Utc::now(),
                data,
                aggregate_id: aggregate.id,
                metadata: self.metadata.clone(),
            },
            (),
        ));
    }
}
