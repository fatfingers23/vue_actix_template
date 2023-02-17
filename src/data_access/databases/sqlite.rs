use std::env;

use diesel::{
    prelude::*,
    r2d2::{self, ConnectionManager},
    sqlite::SqliteConnection,
};
use dotenv::dotenv;

pub type Pool<T> = r2d2::Pool<ConnectionManager<T>>;
pub type SqlLitePool = Pool<diesel::sqlite::SqliteConnection>;
pub type DBConn = SqlLitePool;

pub fn db_pool() -> DBConn {
    dotenv().ok();

    // set up database connection pool
    let conn_spec = match env::var("DATABASE_URL") {
        Ok(db_name) => db_name,
        Err(_) => "local_sqlite.db".to_string(),
    };
    let manager = ConnectionManager::<SqliteConnection>::new(conn_spec);
    Pool::builder()
        .build(manager)
        .expect("Failed to create pool")
}
