use diesel::{
    prelude::*,
    r2d2::{ConnectionManager, Pool, PoolError},
    sqlite::SqliteConnection,
};

use crate::prng;

embed_migrations!();

pub type SqlitePool = Pool<ConnectionManager<SqliteConnection>>;

fn init_pool(database_url: &str) -> Result<SqlitePool, PoolError> {
    let manager = ConnectionManager::<SqliteConnection>::new(database_url);
    Pool::builder().build(manager)
}

pub fn establish_connection(database_url: &str) -> SqlitePool {
    let conn = SqliteConnection::establish(database_url).unwrap();
    embedded_migrations::run(&conn).unwrap();
    prng::insert_prng_if_not_exists(&conn).unwrap();
    init_pool(database_url).unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}
