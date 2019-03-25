use crate::{db, config::Config};
use actix::Addr;
use rusoto_s3::S3Client;

#[derive(Clone)]
pub struct State{
    pub db: Addr<db::DbActor>,
    pub config: Config,
    pub s3_client: S3Client,
}
