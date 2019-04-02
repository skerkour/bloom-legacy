use rusoto_s3::{
    PutObjectRequest,
    S3,
};
use diesel::{
    PgConnection,
    r2d2::{PooledConnection, ConnectionManager},
};
use kernel::{
    events::EventMetadata,
    KernelError,
};
use futures::future::Future;
use std::str::FromStr;
use crate::{
    domain::upload_session,
    validators,
};


#[derive(Clone, Debug)]
pub struct Complete {
    pub s3_bucket: String,
    pub metadata: EventMetadata,
}

impl<'a> eventsourcing::Command<'a> for Complete {
    type Aggregate = upload_session::UploadSession;
    type Event = upload_session::Event;
    type Context = rusoto_s3::S3Client;
    type Error = KernelError;
    type NonStoredData = ();

    fn validate(&self, _ctx: &Self::Context, _aggregate: &Self::Aggregate) -> Result<(), Self::Error> {

        return Ok(());
    }


    fn build_event(&self, _ctx: &Self::Context, aggregate: &Self::Aggregate) -> Result<(Self::Event, Self::NonStoredData), Self::Error> {
        // TODO: HEAD request

        let event_data = upload_session::EventData::CompletedV1(upload_session::CompletedV1{
            size: 0,
            type_: "application/octet-stream".to_string(),
        });

        return  Ok((upload_session::Event{
            id: uuid::Uuid::new_v4(),
            timestamp: chrono::Utc::now(),
            data: event_data,
            aggregate_id: aggregate.id,
            metadata: self.metadata.clone(),
        }, ()));
    }
}
