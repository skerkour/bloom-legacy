use crate::{domain::file, FOLDER_TYPE};
use diesel::{
    r2d2::{ConnectionManager, PooledConnection},
    PgConnection,
};
use futures::future::Future;
use kernel::{events::EventMetadata, KernelError};
use rusoto_core::Region;
use rusoto_credential::{EnvironmentProvider, ProvideAwsCredentials};
use rusoto_s3::{util::PreSignedRequest, GetObjectRequest};
use std::str::FromStr;

#[derive(Clone, Debug)]
pub struct Download {
    pub file_id: uuid::Uuid,
    pub owner_id: uuid::Uuid,
    pub s3_bucket: String,
    pub s3_region: String,
    pub metadata: EventMetadata,
}

impl eventsourcing::Command for Download {
    type Aggregate = file::File;
    type Event = file::Event;
    type Context = PooledConnection<ConnectionManager<PgConnection>>;
    type Error = KernelError;
    type NonStoredData = String;

    fn validate(
        &self,
        _ctx: &Self::Context,
        aggregate: &Self::Aggregate,
    ) -> Result<(), Self::Error> {
        if aggregate.type_ == FOLDER_TYPE {
            return Err(KernelError::Validation(
                "You can't download a folder".to_string(),
            ));
        }
        return Ok(());
    }

    fn build_event(
        &self,
        _ctx: &Self::Context,
        aggregate: &Self::Aggregate,
    ) -> Result<(Self::Event, Self::NonStoredData), Self::Error> {
        let key = format!("drive/{}/{}", self.owner_id, self.file_id);
        let req = GetObjectRequest {
            bucket: self.s3_bucket.clone(),
            key: key,
            response_content_disposition: Some(format!(
                r#"attachment; filename="{}""#,
                &aggregate.name
            )),
            response_content_type: Some(aggregate.type_.clone()),
            ..Default::default()
        };
        // TODO: handle error
        let region = Region::from_str(&self.s3_region).expect("AWS region not valid");
        let credentials = EnvironmentProvider::default()
            .credentials()
            .wait()
            .expect("error getting default credentials");
        let presigned_url = req.get_presigned_url(&region, &credentials, &Default::default());

        let event_data = file::EventData::DownloadedV1(file::DownloadedV1 {
            presigned_url: presigned_url.clone(),
        });;

        return Ok((
            file::Event {
                id: uuid::Uuid::new_v4(),
                timestamp: chrono::Utc::now(),
                data: event_data,
                aggregate_id: aggregate.id,
                metadata: self.metadata.clone(),
            },
            presigned_url,
        ));
    }
}
