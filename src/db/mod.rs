use dotenv::dotenv;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use std::env;
use tokio::sync::OnceCell;

pub mod user;

async fn init_pool() -> Pool<Postgres> {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .expect("Error connecting to database")
}

static DB_POOL: OnceCell<Pool<Postgres>> = OnceCell::const_new();

pub async fn get_db_pool() -> &'static Pool<Postgres> {
    DB_POOL.get_or_init(init_pool).await
}
