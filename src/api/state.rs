use crate::db;
use actix::Addr;

#[derive(Clone)]
pub struct State{
    pub db: Addr<db::DbActor>,
}
