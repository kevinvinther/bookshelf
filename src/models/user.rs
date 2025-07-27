use derive_builder::Builder;
use sqlx::MySqlPool;

#[derive(Debug, Builder)]
pub struct User {
    #[builder(default = 0)]
    pub id: i64,
    pub name: String,
    pub email: String,
    pub password: String,
}

impl User {

    /// Insert user into the DB
    pub async fn create(&mut self, pool: &MySqlPool) -> Result<(), sqlx::Error> {
        let res = sqlx::query!(
            "INSERT INTO users (name, email, password) VALUES (?, ?, ?)",
            self.name,
            self.email,
            self.password
        )
        .execute(pool)
        .await?;

        self.id = res.last_insert_id() as i64;

        Ok(())
    }

    /// Update this user row in place.
    pub async fn update(&self, pool: &MySqlPool) -> Result<u64, sqlx::Error> {
        let updated = sqlx::query!(
            "UPDATE users SET name = ?, email = ?, password = ? WHERE id = ?",
            self.name,
            self.email,
            self.password,
            self.id
        )
        .execute(pool)
        .await?;
        Ok(updated.rows_affected())
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

    /// Deletes the row associated with the record
    pub async fn delete(&self, pool: &MySqlPool) -> Result<u64, sqlx::Error> {
        let deleted = sqlx::query_as!(User, "DELETE FROM users WHERE id = ?", self.id)
            .execute(pool)
            .await?;

        Ok(deleted.rows_affected())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::setup_db;

    #[tokio::test]
    async fn create_then_get_and_delete() {
        // Arrange
        let pool = setup_db().await;
        let mut u = UserBuilder::default()
            .name("Test".to_string())
            .email("a@create.com".to_string())
            .password("pw".to_string())
            .build()
            .unwrap();

        // Act
        u.create(&pool).await.unwrap();
        let fetched = User::get(&pool, u.id).await.unwrap();
        let deleted_rows = u.delete(&pool).await.unwrap();

        // Assert

        // Asserts the db has the correct values
        assert_eq!("a@create.com", fetched.email);
        assert_eq!("Test", fetched.name);
        assert_eq!("pw", fetched.password);

        // Asserts the struct has the same values as the db
        assert_eq!(u.id, fetched.id);
        assert_eq!(u.email, fetched.email);
        assert_eq!(u.name, fetched.name);
        assert_eq!(u.password, fetched.password);

        // Asserts that one row is deleted (i.e. the one we just set up
        assert_eq!(1, deleted_rows)
    }

    #[tokio::test]
    async fn update() {
        // Arrange
        let pool = setup_db().await;
        let mut u = UserBuilder::default()
            .name("Test".to_string())
            .email("a@update.com".to_string())
            .password("pw".to_string())
            .build()
            .unwrap();

        // Act
        u.create(&pool).await.unwrap();
        let fetched = User::get(&pool, u.id).await.unwrap();

        u.name = "New Name".to_string();
        u.email = "b@update.dk".to_string();
        u.password = "npw".to_string();

        let updated_rows = u.update(&pool).await.unwrap();

        let updated_fetch = User::get(&pool, u.id).await.unwrap();

        let deleted_rows = u.delete(&pool).await.unwrap();

        // Assert
        assert_ne!(fetched.name, updated_fetch.name);
        assert_ne!(fetched.email, updated_fetch.email);
        assert_ne!(fetched.password, updated_fetch.password);
        assert_eq!(fetched.id, updated_fetch.id);

        assert_eq!(1, updated_rows);
        assert_eq!(1, deleted_rows);
    }
}
