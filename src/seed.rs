use fake::faker::internet::en::FreeEmail;
use fake::faker::name::en::Name;
use fake::{Fake, Faker};
use sqlx::PgPool;

pub async fn seed_users(pool: &PgPool, count: usize) -> Result<(), sqlx::Error> {
    for _ in 0..count {
        let name: String = Name().fake();
        let email: String = FreeEmail().fake();
        let password: String = Faker.fake(); // random string

        sqlx::query!(
            "INSERT INTO users (name, email, password) VALUES ($1, $2, $3)",
            name,
            email,
            password
        )
        .execute(pool)
        .await?;
    }
    println!("🌱 Seeded {} users!", count);
    Ok(())
}
