use rusoto_s3::{
    PutObjectRequest, StreamingBody, S3,
};
use diesel::{
    PgConnection,
    r2d2::{PooledConnection, ConnectionManager},
};
use kernel::{
    events::EventMetadata,
    KernelError,
};
use std::fs;
use futures_fs::FsPool;
use std::io::Read;
use crate::{
    domain::upload,
};


#[derive(Clone)]
pub struct Complete {
    pub s3_bucket: String,
    pub s3_client: rusoto_s3::S3Client,
    pub file_path: String,
    pub metadata: EventMetadata,
}

impl eventsourcing::Command for Complete {
    type Aggregate = upload::Upload;
    type Event = upload::Event;
    type Context = PooledConnection<ConnectionManager<PgConnection>>;
    type Error = KernelError;
    type NonStoredData = ();

    fn validate(&self, _ctx: &Self::Context, _aggregate: &Self::Aggregate) -> Result<(), Self::Error> {

        return Ok(());
    }


    fn build_event(&self, _ctx: &Self::Context, aggregate: &Self::Aggregate) -> Result<(Self::Event, Self::NonStoredData), Self::Error> {
        // get file metadata

        let file_metadata = fs::metadata(&self.file_path)?;
        let content_type = {
            // read first 512 bytes to detect content type
            let mut contents = [0u8;512];
            let mut file = fs::File::open(&self.file_path)?;
            file.read(&mut contents)?;
            mimesniff::detect_content_type(&contents)
        };

        let fspool = FsPool::default();
        let file_stream = fspool.read(self.file_path.clone(), Default::default());
        let req = PutObjectRequest {
            bucket: self.s3_bucket.clone(),
            key: format!("drive/{}/{}", aggregate.owner_id, aggregate.file_id),
            content_length: Some(file_metadata.len() as i64),
            content_type: Some(content_type.to_string()),
            body: Some(StreamingBody::new(file_stream)),
            ..Default::default()
        };
        self.s3_client.put_object(req).sync().expect("pahser: Couldn't PUT object");

        let event_data = upload::EventData::CompletedV1(upload::CompletedV1{
            size: file_metadata.len() as i64,
            type_: content_type.to_string(),
        });

        return  Ok((upload::Event{
            id: uuid::Uuid::new_v4(),
            timestamp: chrono::Utc::now(),
            data: event_data,
            aggregate_id: aggregate.id,
            metadata: self.metadata.clone(),
        }, ()));
    }
}
