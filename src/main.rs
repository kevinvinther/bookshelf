use anyhow::Result;
use bookshelf::{db::init_pool, models::user::User};
use fake::faker::internet::en::FreeEmail;
use fake::faker::name::en::Name;
use fake::{Fake, Faker};
use bookshelf::models::user::UserBuilder;

mod seed;
use seed::seed_users;

#[tokio::main]
async fn main() -> Result<()> {
    let pool = init_pool().await?;
    seed_users(&pool, 10).await?;

    // Demo
    let mut u = UserBuilder::default()
        .name(Name().fake())
        .email(FreeEmail().fake())
        .password(Faker.fake())
        .build()?;
    

    u.create(&pool).await?;
    u.name = "John".into();
    u.update(&pool).await?;
    println!("🧍 {:?}", User::get(&pool, u.id).await?);

    Ok(())
}
