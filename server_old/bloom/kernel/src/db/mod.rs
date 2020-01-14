mod actor;

use actix::{Addr, SyncArbiter};
use diesel::{
    prelude::PgConnection,
    r2d2::{ConnectionManager, Pool},
};
// use diesel_derive_enum::
use crate::config;

pub mod schema;
pub use actor::DbActor;

pub fn init(cfg: &config::Config) -> Addr<DbActor> {
    let conn = get_pool_db_conn(cfg);
    // Start POOL_SIZE `DbActor ` actors, each with its own database
    // connection, and each in its own thread
    SyncArbiter::start(bloom_const::db::ACTOR_POOL_SIZE as usize, move || DbActor {
        pool: conn.clone(),
    })
}

pub fn get_pool_db_conn(cfg: &config::Config) -> Pool<ConnectionManager<PgConnection>> {
    let manager = ConnectionManager::<PgConnection>::new(cfg.database.url.as_str());
    Pool::builder()
        .max_size(bloom_const::db::PG_POOL_SIZE)
        .build(manager)
        .expect("Failed to create pool.")
}
