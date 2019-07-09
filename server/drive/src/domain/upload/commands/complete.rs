use crate::domain::upload;
use diesel::{
    r2d2::{ConnectionManager, PooledConnection},
    PgConnection,
};
use eventsourcing::{Event, EventTs};
use futures_fs::FsPool;
use kernel::KernelError;
use rusoto_s3::{PutObjectRequest, StreamingBody, S3};
use std::fs;
use std::io::Read;

#[derive(Clone)]
pub struct Complete {
    pub s3_bucket: String,
    pub s3_client: rusoto_s3::S3Client,
    pub file_path: String,
}

impl eventsourcing::Command for Complete {
    type Aggregate = upload::Upload;
    type Event = Completed;
    type Context = PooledConnection<ConnectionManager<PgConnection>>;
    type Error = KernelError;

    fn validate(
        &self,
        _ctx: &Self::Context,
        _aggregate: &Self::Aggregate,
    ) -> Result<(), Self::Error> {
        return Ok(());
    }

    #[allow(clippy::unused_io_amount)]
    fn build_event(
        &self,
        _ctx: &Self::Context,
        aggregate: &Self::Aggregate,
    ) -> Result<Self::Event, Self::Error> {
        // get file metadata

        let file_metadata = fs::metadata(&self.file_path)?;
        let content_type = {
            // read first 512 bytes to detect content type
            let mut contents = [0u8; 512];
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
        self.s3_client
            .put_object(req)
            .sync()
            .expect("pahser: Couldn't PUT object");

        return Ok(Completed {
            timestamp: chrono::Utc::now(),
            size: file_metadata.len() as i64,
            type_: content_type.to_string(),
        });
    }
}

// Event
#[derive(Clone, Debug, EventTs)]
pub struct Completed {
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub size: i64,
    pub type_: String,
}

impl Event for Completed {
    type Aggregate = upload::Upload;

    fn apply(&self, aggregate: Self::Aggregate) -> Self::Aggregate {
        return Self::Aggregate {
            size: self.size,
            type_: self.type_.clone(),
            ..aggregate
        };
    }
}
