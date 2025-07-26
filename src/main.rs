use anyhow::Result;
use bookshelf::{db::init_pool, models::user::User};
use fake::faker::internet::en::FreeEmail;
use fake::faker::name::en::Name;
use fake::{Fake, Faker};

mod seed;
use seed::seed_users;

#[tokio::main]
async fn main() -> Result<()> {
    let pool = init_pool().await?;
    seed_users(&pool, 10).await?;

    // Demo
    let mut u = User::new(0, Name().fake(), FreeEmail().fake(), Faker.fake());

    u.create(&pool).await?;
    u.name = "John".into();
    u.update(&pool).await?;
    println!("🧍 {:?}", User::get(&pool, u.id).await?);

    Ok(())
}
