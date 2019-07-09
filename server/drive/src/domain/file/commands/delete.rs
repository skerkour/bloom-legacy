use crate::domain::file;
use diesel::{
    r2d2::{ConnectionManager, PooledConnection},
    PgConnection,
};
use eventsourcing::{Event, EventTs};
use kernel::KernelError;
use rusoto_s3::{DeleteObjectRequest, S3};

#[derive(Clone)]
pub struct Delete {
    pub s3_bucket: String,
    pub s3_client: rusoto_s3::S3Client,
}

impl eventsourcing::Command for Delete {
    type Aggregate = file::File;
    type Event = Deleted;
    type Context = PooledConnection<ConnectionManager<PgConnection>>;
    type Error = KernelError;

    fn validate(
        &self,
        _ctx: &Self::Context,
        _aggregate: &Self::Aggregate,
    ) -> Result<(), Self::Error> {
        return Ok(());
    }

    fn build_event(
        &self,
        _ctx: &Self::Context,
        aggregate: &Self::Aggregate,
    ) -> Result<Self::Event, Self::Error> {
        if aggregate.type_ != crate::FOLDER_TYPE {
            let req = DeleteObjectRequest {
                bucket: self.s3_bucket.clone(),
                key: format!("drive/{}/{}", aggregate.owner_id, aggregate.id),
                ..Default::default()
            };
            self.s3_client
                .delete_object(req)
                .sync()
                .expect("drive: Couldn't Delete object");
        }

        return Ok(Deleted {
            timestamp: chrono::Utc::now(),
        });
    }
}

// Event
#[derive(Clone, Debug, EventTs)]
pub struct Deleted {
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

impl Event for Deleted {
    type Aggregate = file::File;

    fn apply(&self, aggregate: Self::Aggregate) -> Self::Aggregate {
        return aggregate;
    }
}
