use diesel::r2d2::{ Pool, ConnectionManager };
use diesel::{PgConnection};

pub struct DbActor{
    pub pool: Pool<ConnectionManager<PgConnection>>,
}
