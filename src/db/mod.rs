mod actor;

use actix::{Addr, SyncArbiter, Actor};
use diesel::{
    prelude::PgConnection,
    r2d2::{ Pool, ConnectionManager },
};


pub use actor::DbActor;
const POOL_SIZE: u32 = 20;


pub fn init() -> Addr<DbActor> {
    let db_url = dotenv::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(db_url);
    let conn = Pool::builder().max_size(POOL_SIZE).build(manager).expect("Failed to create pool.");
    // Start POOL_SIZE `DbActor ` actors, each with its own database
    // connection, and each in its own thread
    SyncArbiter::start((POOL_SIZE * 3) as usize, move || {
        DbActor{
            pool: conn.clone()}
    })
}

