// Table for a many-to-many relationship between a user and a book, holding
// information that a specific user has on a specific book

use crate::models::book::Book;
use crate::models::user::User;
use chrono::{DateTime, Utc};
use derive_builder::Builder;
use fake::rand;
use sqlx::MySqlPool;

#[derive(Debug, Clone, sqlx::Type, PartialEq)]
#[sqlx(type_name = "ENUM")]
pub enum ReadingStatus {
    #[sqlx(rename = "to-read")]
    ToRead,
    #[sqlx(rename = "reading")]
    Reading,
    #[sqlx(rename = "completed")]
    Completed,
}

impl From<String> for ReadingStatus {
    fn from(s: String) -> Self {
        match s.as_str() {
            "to-read" => ReadingStatus::ToRead,
            "reading" => ReadingStatus::Reading,
            "completed" => ReadingStatus::Completed,
            other => panic!("Invalid ReadingStatus from DB: {other}"),
        }
    }
}

impl fake::Dummy<fake::Faker> for ReadingStatus {
    fn dummy_with_rng<R: rand::Rng + ?Sized>(_: &fake::Faker, rng: &mut R) -> Self {
        match rng.random_range(0..=2) {
            0 => ReadingStatus::ToRead,
            1 => ReadingStatus::Reading,
            _ => ReadingStatus::Completed,
        }
    }
}

#[derive(Debug, Builder, sqlx::FromRow)]
pub struct UserBook {
    user_id: i64,
    book_id: i64,
    #[builder(default = ReadingStatus::ToRead)]
    status: ReadingStatus,
    #[builder(default = None)]
    rating: Option<u8>,
    #[builder(default = Utc::now())]
    added_at: DateTime<Utc>,
    #[builder(default = None)]
    began_reading: Option<DateTime<Utc>>,
    #[builder(default = None)]
    done_reading: Option<DateTime<Utc>>,
    #[builder(default = None)]
    current_page: Option<u32>,
}

impl UserBook {
    /// Creates a new instance of `UserBook` and adds it to the database.
    pub async fn create(&mut self, pool: &MySqlPool) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "INSERT INTO user_books (user_id, book_id, status, rating, added_at, began_reading, done_reading, current_page) VALUES (?, ?, ?, ?, ?, ?, ?, ?)",
            self.user_id,
            self.book_id,
            self.status,
            self.rating,
            self.added_at,
            self.began_reading,
            self.done_reading,
            self.current_page
        )
            .execute(pool)
            .await?;

        Ok(())
    }

    /// Synchronizes the information in the struct to the database.
    pub async fn update(&mut self, pool: &MySqlPool) -> Result<u64, sqlx::Error> {
        let updated = sqlx::query!(
            "UPDATE user_books SET user_id = ?, book_id = ?, status = ?, rating = ?, added_at = ?, began_reading = ?, done_reading = ?, current_page = ? WHERE user_id = ? AND book_id = ?",
            self.user_id,
            self.book_id,
            self.status,
            self.rating,
            self.added_at,
            self.began_reading,
            self.done_reading,
            self.current_page,
            self.user_id,
            self.book_id,
        )
        .execute(pool)
        .await?;

        Ok(updated.rows_affected())
    }

    /// Synchronizes the struct with the information in the database.
    pub async fn fetch(&self, pool: &MySqlPool) -> Result<Self, sqlx::Error> {
        let record = sqlx::query_as!(
            UserBook,
            "SELECT user_id, book_id, status, rating, added_at, began_reading, done_reading, current_page FROM user_books WHERE book_id = ? AND user_id = ?",
            self.book_id,
            self.user_id
        )
            .fetch_one(pool)
            .await?;

        Ok(record)
    }

    /// Gets a specific `user_book` instance from the database given a `book_id`
    /// and `user_id`.
    pub async fn get(pool: &MySqlPool, book_id: i64, user_id: i64) -> Result<Self, sqlx::Error> {
        let record = sqlx::query_as!(
            UserBook,
            "SELECT user_id, book_id, status, rating, added_at, began_reading, done_reading, current_page FROM user_books WHERE book_id = ? AND user_id = ?",
            book_id,
            user_id
        )
            .fetch_one(pool)
            .await?;

        Ok(record)
    }

    /// Deletes the row in the database associated with this instance.
    pub async fn delete(&self, pool: &MySqlPool) -> Result<u64, sqlx::Error> {
        let deleted = sqlx::query!(
            "DELETE FROM user_books WHERE user_id = ? AND book_id = ?",
            self.user_id,
            self.book_id
        )
        .execute(pool)
        .await?;

        Ok(deleted.rows_affected())
    }

    /// Gets the user associated with this instance.
    pub async fn get_user(&self, pool: &MySqlPool) -> Result<User, sqlx::Error> {
        let user = sqlx::query_as!(
            User,
            "SELECT id, name, email, password FROM users WHERE id = ?",
            self.user_id
        )
        .fetch_one(pool)
        .await?;

        Ok(user)
    }

    /// Gets the book associated with this user.
    pub async fn get_book(&self, pool: &MySqlPool) -> Result<Book, sqlx::Error> {
        let book = sqlx::query_as!(
            Book,
            "SELECT id, title, author, isbn, published_year, description, cover_url, pages FROM books WHERE id = ?",
            self.book_id
        )
            .fetch_one(pool)
            .await?;

        Ok(book)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::factories;
    use crate::models::book::BookBuilder;
    use crate::models::user::UserBuilder;
    use crate::test_utils::setup_db;
    use chrono::Timelike;

    #[tokio::test]
    async fn create_then_get_and_delete() {
        // Arrange
        let pool = setup_db().await;
        let mut u = UserBuilder::default()
            .name("Test".to_string())
            .email("user_book@create.com".to_string())
            .password("pw".to_string())
            .build()
            .unwrap();

        u.create(&pool).await.unwrap();

        let mut b = BookBuilder::default()
            .title("Crime and Punishment".to_string())
            .author("Fyodor Dostoevsky".to_string())
            .isbn("9780307829605".to_string())
            .published_year(2012)
            .description(Some("FYODOR MIKAILOVICH DOSTOEVSKY's life was as dark and dramatic as the great novels he wrote. He was born in Moscow in 1821. A short first novel, Poor Folk (1846) brought him instant success, but his writing career was cut short by his arrest for alleged subversion against Tsar Nicholas I in 1849. In prison he was given the “silent treatment” for eight months (guards even wore velvet soled boots) before he was led in front a firing squad. Dressed in a death shroud, he faced an open grave and awaited execution, when suddenly, an order arrived commuting his sentence. He then spent four years at hard labor in a Siberian prison, where he began to suffer from epilepsy, and he returned to St. Petersburg only a full ten years after he had left in chains.\n His prison experiences coupled with his conversion to a profoundly religious philosophy formed the basis for his great novels. But it was his fortuitous marriage to Anna Snitkina, following a period of utter destitution brought about by his compulsive gambling, that gave Dostoevsky the emotional stability to complete Crime and Punishment (1866), The Idiot (1868-69), The Possessed (1871-72),and The Brothers Karamazov (1879-80). When Dostoevsky died in 1881, he left a legacy of masterworks that influenced the great thinkers and writers of the Western world and immortalized him as a giant among writers of world literature.".to_string()))
            .cover_url(None)
            .pages(592)
            .build()
            .unwrap();

        b.create(&pool).await.unwrap();

        let mut user_book = UserBookBuilder::default()
            .book_id(b.id)
            .user_id(u.id)
            .added_at(Utc::now().with_nanosecond(0).unwrap())
            .build()
            .unwrap();

        // Act
        user_book.create(&pool).await.unwrap();
        let fetched = UserBook::get(&pool, user_book.book_id, user_book.user_id)
            .await
            .unwrap();
        let deleted_rows = user_book.delete(&pool).await.unwrap();

        u.delete(&pool).await.unwrap();
        b.delete(&pool).await.unwrap();

        // Assert

        // Asserts the db has the correct values
        assert_eq!(u.id, fetched.user_id);
        assert_eq!(b.id, fetched.book_id);
        assert_eq!(user_book.status, fetched.status);
        assert_eq!(user_book.rating, fetched.rating);
        assert_eq!(user_book.added_at, fetched.added_at);
        assert_eq!(user_book.began_reading, fetched.began_reading);
        assert_eq!(user_book.done_reading, fetched.done_reading);
        assert_eq!(user_book.current_page, fetched.current_page);

        assert_eq!(1, deleted_rows);
    }

    #[tokio::test]
    async fn update() {
        // Arrange
        let pool = setup_db().await;
        let mut u = UserBuilder::default()
            .name("Test".to_string())
            .email("user_book@update.com".to_string())
            .password("pw".to_string())
            .build()
            .unwrap();

        u.create(&pool).await.unwrap();

        let mut b = BookBuilder::default()
            .title("Crime and Punishment".to_string())
            .author("Fyodor Dostoevsky".to_string())
            .isbn("9780307829606".to_string())
            .published_year(2012)
            .description(Some("FYODOR MIKAILOVICH DOSTOEVSKY's life was as dark and dramatic as the great novels he wrote. He was born in Moscow in 1821. A short first novel, Poor Folk (1846) brought him instant success, but his writing career was cut short by his arrest for alleged subversion against Tsar Nicholas I in 1849. In prison he was given the “silent treatment” for eight months (guards even wore velvet soled boots) before he was led in front a firing squad. Dressed in a death shroud, he faced an open grave and awaited execution, when suddenly, an order arrived commuting his sentence. He then spent four years at hard labor in a Siberian prison, where he began to suffer from epilepsy, and he returned to St. Petersburg only a full ten years after he had left in chains.\n His prison experiences coupled with his conversion to a profoundly religious philosophy formed the basis for his great novels. But it was his fortuitous marriage to Anna Snitkina, following a period of utter destitution brought about by his compulsive gambling, that gave Dostoevsky the emotional stability to complete Crime and Punishment (1866), The Idiot (1868-69), The Possessed (1871-72),and The Brothers Karamazov (1879-80). When Dostoevsky died in 1881, he left a legacy of masterworks that influenced the great thinkers and writers of the Western world and immortalized him as a giant among writers of world literature.".to_string()))
            .cover_url(None)
            .pages(592)
            .build()
            .unwrap();

        b.create(&pool).await.unwrap();

        let mut user_book = UserBookBuilder::default()
            .book_id(b.id)
            .user_id(u.id)
            .added_at(Utc::now().with_nanosecond(0).unwrap())
            .build()
            .unwrap();

        // Act
        user_book.create(&pool).await.unwrap();
        let fetched = UserBook::get(&pool, user_book.book_id, user_book.user_id)
            .await
            .unwrap();

        user_book.rating = Some(5);
        user_book.current_page = Some(7362); // It's a long book!

        let began_reading = Utc::now().with_nanosecond(0).unwrap();
        user_book.began_reading = Some(began_reading);

        let updated_rows = user_book.update(&pool).await.unwrap();
        println!("Updated rows: {}", updated_rows);
        let updated_fetch = UserBook::get(&pool, user_book.book_id, user_book.user_id)
            .await
            .expect("Record should exist after updating");
        let deleted_rows = user_book.delete(&pool).await.unwrap();

        u.delete(&pool).await.unwrap();
        b.delete(&pool).await.unwrap();

        // Assert
        assert_ne!(fetched.rating, updated_fetch.rating);
        assert_ne!(fetched.current_page, updated_fetch.current_page);
        assert_ne!(fetched.began_reading, updated_fetch.began_reading);
        // Sanity Check
        assert_eq!(fetched.done_reading, updated_fetch.done_reading);

        assert_eq!(1, updated_rows);
        assert_eq!(1, deleted_rows);
    }

    #[tokio::test]
    async fn get_book_gets_correct_book() {
        // Arrange
        let pool = setup_db().await;

        let mut book = factories::fake_book();
        book.create(&pool).await.unwrap();
        let mut user = factories::fake_user();
        user.create(&pool).await.unwrap();
        let mut user_book = factories::fake_user_book(user.id, book.id);
        user_book.create(&pool).await.unwrap();

        // Act
        let user_book_received_book = user_book.get_book(&pool).await.unwrap();

        book.delete(&pool).await.unwrap();
        user.delete(&pool).await.unwrap();
        user_book.delete(&pool).await.unwrap();

        // Assert
        assert_eq!(user_book_received_book, book);
    }

    #[tokio::test]
    async fn get_user_gets_correct_user() {
        // Arrange
        let pool = setup_db().await;

        let mut book = factories::fake_book();
        book.create(&pool).await.unwrap();
        let mut user = factories::fake_user();
        user.create(&pool).await.unwrap();
        let mut user_book = factories::fake_user_book(user.id, book.id);
        user_book.create(&pool).await.unwrap();

        // Act
        let user_book_received_user = user_book.get_user(&pool).await.unwrap();

        book.delete(&pool).await.unwrap();
        user.delete(&pool).await.unwrap();
        user_book.delete(&pool).await.unwrap();

        // Assert
        assert_eq!(user_book_received_user, user);
    }
}
