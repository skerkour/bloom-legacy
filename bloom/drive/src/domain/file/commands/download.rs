use crate::{domain::file, FOLDER_TYPE};
use diesel::{
    r2d2::{ConnectionManager, PooledConnection},
    PgConnection,
};
use eventsourcing::{Event, EventTs};
use futures::future::Future;
use kernel::KernelError;
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
}

impl eventsourcing::Command for Download {
    type Aggregate = file::File;
    type Event = Downloaded;
    type Context = PooledConnection<ConnectionManager<PgConnection>>;
    type Error = KernelError;

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
    ) -> Result<Self::Event, Self::Error> {
        let key = format!("drive/{}/{}", self.owner_id, self.file_id);
        let req = GetObjectRequest {
            bucket: self.s3_bucket.clone(),
            key,
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

        return Ok(Downloaded {
            timestamp: chrono::Utc::now(),
            presigned_url: presigned_url.clone(),
        });
    }
}

// Event
#[derive(Clone, Debug, EventTs)]
pub struct Downloaded {
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub presigned_url: String,
}

impl Event for Downloaded {
    type Aggregate = file::File;

    fn apply(&self, aggregate: Self::Aggregate) -> Self::Aggregate {
        return aggregate;
    }
}
