use dotenvy::dotenv;
use sqlx::PgPool;
use sqlx::migrate;
use std::env;

/// Set up a clean DB pool for testing.
pub async fn setup_db() -> PgPool {
    dotenv().ok();
    let url = env::var("TEST_DATABASE_URL").expect("TEST_DATABASE_URL must be set in .env");

    let pool = PgPool::connect(&url).await.expect("Failed to connect");

    migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Migrations failed");

    pool
}
