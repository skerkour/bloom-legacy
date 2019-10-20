use crate::{domain::file, FOLDER_TYPE};
use diesel::{
    r2d2::{ConnectionManager, PooledConnection},
    PgConnection,
};
use eventsourcing::{Event, EventTs};
use kernel::KernelError;
use rusoto_s3::{CopyObjectRequest, S3Client, S3};

#[derive(Clone)]
pub struct Copy_ {
    pub to: uuid::Uuid,
    pub new_file: uuid::Uuid,
    pub s3_client: S3Client,
    pub s3_bucket: String,
}

impl eventsourcing::Command for Copy_ {
    type Aggregate = file::File;
    type Event = Copied;
    type Context = PooledConnection<ConnectionManager<PgConnection>>;
    type Error = KernelError;

    // TODO: check that to is owned by owner
    fn validate(
        &self,
        _ctx: &Self::Context,
        aggregate: &Self::Aggregate,
    ) -> Result<(), Self::Error> {
        if aggregate.trashed_at.is_some() {
            return Err(KernelError::Validation(
                "File cannot be copied while in Trash".to_string(),
            ));
        }

        if aggregate.type_ == FOLDER_TYPE {
            return Err(KernelError::Validation(
                "Folders cannot be copied".to_string(),
            ));
        }

        return Ok(());
    }

    fn build_event(
        &self,
        _ctx: &Self::Context,
        aggregate: &Self::Aggregate,
    ) -> Result<Self::Event, Self::Error> {
        let source_key = format!("drive/{}/{}", aggregate.owner_id, aggregate.id);
        let dest_key = format!("drive/{}/{}", aggregate.owner_id, self.new_file);
        let req = CopyObjectRequest {
            bucket: self.s3_bucket.clone(),
            key: dest_key,
            copy_source: source_key,
            metadata_directive: Some("REPLACE".to_owned()),
            ..Default::default()
        };
        // TODO: handle error
        self.s3_client
            .copy_object(req)
            .sync()
            .expect("Couldn't PUT object");

        return Ok(Copied {
            timestamp: chrono::Utc::now(),
            to: self.to, // new parent
            new_file: self.new_file,
        });
    }
}

// Event
#[derive(Clone, Debug, EventTs)]
pub struct Copied {
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub to: uuid::Uuid, // new parent
    pub new_file: uuid::Uuid,
}

impl Event for Copied {
    type Aggregate = file::File;

    fn apply(&self, aggregate: Self::Aggregate) -> Self::Aggregate {
        return aggregate;
    }
}

// use rusoto_s3::{
//     PutObjectRequest,
//     util::PreSignedRequest,
// };
// use rusoto_core::{Region};
// use rusoto_credential::{EnvironmentProvider, ProvideAwsCredentials};
// use diesel::{
//     PgConnection,
//     r2d2::{PooledConnection, ConnectionManager},
// };
// use kernel::{
//     events::EventMetadata,
//     KernelError,
// };
// use futures::future::Future;
// use std::str::FromStr;
// use crate::{
//     domain::upload_session,
//     validators,
// };

// #[derive(Clone, Debug)]
// pub struct Start {
//     pub file_name: String,
//     pub parent_id: Option<uuid::Uuid>,
//     pub owner_id: uuid::Uuid,
//     pub s3_bucket: String,
//     pub s3_region: String,
//     pub metadata: EventMetadata,
// }

// impl eventsourcing::Command for Start {
//     type Aggregate = upload_session::UploadSession;
//     type Event = upload_session::Event;
//     type Context = PooledConnection<ConnectionManager<PgConnection>>;
//     type Error = KernelError;
//     type NonStoredData = ();

//     fn validate(&self, _ctx: &Self::Context, _aggregate: &Self::Aggregate) -> Result<(), Self::Error> {
//         validators::file_name(&self.file_name)?;
//         // TODO: check that parent exists

//         return Ok(());
//     }

//     fn build_event(&self, _ctx: &Self::Context, _aggregate: &Self::Aggregate) -> Result<(Self::Event, Self::NonStoredData), Self::Error> {
//         // presign request
//         let id = uuid::Uuid::new_v4();
//         let file_id = uuid::Uuid::new_v4();
//         let key = format!("drive/{}/{}", self.owner_id, file_id);

//         let req = PutObjectRequest {
//             bucket: self.s3_bucket.clone(),
//             key: key,
//             ..Default::default()
//         };
//         // TODO: handle error
//         let region = Region::from_str(&self.s3_region).expect("AWS region not valid");
//         let credentials = EnvironmentProvider::default()
//             .credentials()
//             .wait()
//             .expect("error getting default credentials");
//         let presigned_url = req.get_presigned_url(&region, &credentials, &Default::default());

//         let event_data = upload_session::EventData::StartedV1(upload_session::StartedV1{
//             id,
//             file_name: self.file_name.clone(),
//             file_id,
//             parent_id: self.parent_id,
//             owner_id: self.owner_id,
//             presigned_url,
//         });

//         return  Ok((upload_session::Event{
//             id: uuid::Uuid::new_v4(),
//             timestamp: chrono::Utc::now(),
//             data: event_data,
//             aggregate_id: id,
//             metadata: self.metadata.clone(),
//         }, ()));
//     }
// }
