use crate::{db, config};
use actix::Addr;

#[derive(Clone)]
pub struct State{
    pub db: Addr<db::DbActor>,
    pub config: config::Config,
}
