use diesel::sqlite::SqliteConnection;
use diesel::r2d2::{ConnectionManager, Pool, PoolError};
use dotenv::dotenv;
use std::env;

pub type SqlitePool = Pool<ConnectionManager<SqliteConnection>>;

fn init_pool(database_url: &str) -> Result<SqlitePool, PoolError> {
    let manager = ConnectionManager::<SqliteConnection>::new(database_url);
    Pool::builder().build(manager)
}

pub fn establish_connection() -> SqlitePool {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    init_pool(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}