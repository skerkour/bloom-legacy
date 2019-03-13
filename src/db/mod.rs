mod actor;

use actix::{Addr, SyncArbiter};
use diesel::{
    prelude::PgConnection,
    r2d2::{ Pool, ConnectionManager },
};

pub mod schema;
pub use actor::DbActor;
pub static PG_POOL_SIZE: u32 = 20;
pub static ACTOR_POOL_SIZE: u32 = PG_POOL_SIZE * 3;


pub fn init() -> Addr<DbActor> {
    let db_url = dotenv::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(db_url);
    let conn = Pool::builder().max_size(PG_POOL_SIZE).build(manager).expect("Failed to create pool.");
    // Start POOL_SIZE `DbActor ` actors, each with its own database
    // connection, and each in its own thread
    SyncArbiter::start(ACTOR_POOL_SIZE as usize, move || {
        DbActor{
            pool: conn.clone()}
    })
}

