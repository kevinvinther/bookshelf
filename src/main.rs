use anyhow::Result;
use dotenvy::dotenv;
use sqlx::{MySqlPool, mysql::MySqlPoolOptions};
use std::env;

mod seed;
use seed::seed_users;

#[derive(Debug)]
struct User {
    pub id: i64,
    pub name: String,
    pub email: String,
    pub password: String,
}

impl User {
    /// Insert user into the DB, returning the new given ID
    pub async fn create(&mut self, pool: &MySqlPool) -> Result<i64, sqlx::Error> {
        let res = sqlx::query!(
            "INSERT INTO users (name, email, password) VALUES (?, ?, ?)",
            self.name,
            self.email,
            self.password
        )
        .execute(pool)
        .await?;
        Ok(res.last_insert_id().try_into().unwrap())
    }

    /// Update this user row in place.
    pub async fn update(&self, pool: &MySqlPool) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "UPDATE users SET name = ?, email = ?, password = ? WHERE id = ?",
            self.name,
            self.email,
            self.password,
            self.id
        )
        .execute(pool)
        .await?;
        Ok(())
    }

    /// Fetches the user with the id from self.
    pub async fn fetch(&self, pool: &MySqlPool) -> Result<Self, sqlx::Error> {
        let record = sqlx::query_as!(
            User,
            "SELECT id, name, email, password FROM users WHERE id = ?",
            self.id
        )
        .fetch_one(pool)
        .await?;
        Ok(record)
    }

    /// Fetch a user by its ID.
    pub async fn get(pool: &MySqlPool, id: i64) -> Result<Self, sqlx::Error> {
        let record = sqlx::query_as!(
            User,
            "SELECT id, name, email, password FROM users WHERE id = ?",
            id
        )
        .fetch_one(pool)
        .await?;
        Ok(record)
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL")?;

    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await?;

    sqlx::migrate!("./migrations").run(&pool).await?;

    seed_users(&pool, 10).await?;

    let mut user = User::get(&pool, 1).await?;

    user.name = "Kevin".to_string();

    user.update(&pool).await?;

    let fetched = user.fetch(&pool).await?;
    println!("🧍 Fetched user: {:?}", fetched);

    Ok(())
}
