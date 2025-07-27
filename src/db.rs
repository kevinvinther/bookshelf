use dotenvy::dotenv;
use sqlx::PgPool;
use std::env;

/// Connects, runs migrations, and returns a pool
pub async fn init_pool() -> Result<PgPool, sqlx::Error> {
    dotenv().ok();
    let url = env::var("DATABASE_URL").expect("DATABASE_URL environment variable not set.");
    let pool = PgPool::connect(&url).await?;
    sqlx::migrate!("./migrations").run(&pool).await?;
    Ok(pool)
}
