use diesel_async::pg::AsyncPgConnection;
use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use std::env;

pub type Pool<T> = bb8::Pool<AsyncDieselConnectionManager<T>>;
pub type PostgresPool = Pool<AsyncPgConnection>;
pub type DBConn = PostgresPool;

pub async fn db_pool() -> DBConn {
    let database_url =
        env::var("DATABASE_URL").expect(&*format!("{value} must be set", value = "DATABASE_URL"));

    let config = AsyncDieselConnectionManager::<diesel_async::AsyncPgConnection>::new(database_url);
    Pool::builder().build(config).await.unwrap()
}
