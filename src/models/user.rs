use sqlx::MySqlPool;

#[derive(Debug)]
pub struct User {
    pub id: i64,
    pub name: String,
    pub email: String,
    pub password: String,
}

impl User {
    /// Create a new instance of the `User` struct
    pub fn new(id: i64, name: String, email: String, password: String) -> Self {
        Self {
            id,
            name,
            email,
            password,
        }
    }

    /// Insert user into the DB, returning the new given ID
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
