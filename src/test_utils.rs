use dotenvy::dotenv;
use sqlx::migrate;
use sqlx::{MySqlPool, mysql::MySqlPoolOptions};
use std::env;

/// Set up a clean DB pool for testing.
pub async fn setup_db() -> MySqlPool {
    dotenv().ok();
    let url = env::var("TEST_DATABASE_URL").expect("TEST_DATABASE_URL must be set in .env.test");

    let pool = MySqlPoolOptions::new()
        .max_connections(1)
        .connect(&url)
        .await
        .expect("Failed to connect");

    migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Migrations failed");

    pool
}
