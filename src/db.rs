use dotenvy::dotenv;
use sqlx::{MySqlPool, mysql::MySqlPoolOptions};
use std::env;

/// Connects, runs migrations, and returns a pool
pub async fn init_pool() -> Result<MySqlPool, sqlx::Error> {
    dotenv().ok();
    let url = env::var("DATABASE_URL").unwrap();
    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&url)
        .await?;
    sqlx::migrate!("./migrations").run(&pool).await?;
    Ok(pool)
}
