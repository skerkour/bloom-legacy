mod actor;

use actix::{Addr, SyncArbiter};
use diesel::{
    prelude::PgConnection,
    r2d2::{ Pool, ConnectionManager },
};
// use diesel_derive_enum::
use crate::config;

pub mod schema;
pub use actor::DbActor;
pub static PG_POOL_SIZE: u32 = 20;
pub static ACTOR_POOL_SIZE: u32 = PG_POOL_SIZE * 3;


pub fn init(cfg: &config::Config) -> Addr<DbActor> {
    let conn = get_pool_db_conn(cfg);
    // Start POOL_SIZE `DbActor ` actors, each with its own database
    // connection, and each in its own thread
    SyncArbiter::start(ACTOR_POOL_SIZE as usize, move || {
        DbActor{
            pool: conn.clone()}
    })
}

pub fn get_pool_db_conn(cfg: &config::Config) -> Pool<ConnectionManager<PgConnection>> {
    let manager = ConnectionManager::<PgConnection>::new(cfg.database_url().as_str());
    Pool::builder().max_size(PG_POOL_SIZE).build(manager).expect("Failed to create pool.")
}
