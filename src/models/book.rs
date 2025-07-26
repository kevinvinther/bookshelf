use sqlx::MySqlPool;

#[derive(Debug)]
pub struct Book {
    pub id: i64,
    pub title: String,
    pub author: String,
    pub isbn: String,
    pub published_year: i32,
    pub description: Option<String>,
    pub cover_url: Option<String>,
    pub pages: i32,
}

impl Book {
    /// Create a new instance of the `Book` struct
    pub fn new(
        id: i64,
        title: String,
        author: String,
        isbn: String,
        published_year: i32,
        description: Option<String>,
        cover_url: Option<String>,
        pages: i32,
    ) -> Self {
        Self {
            id,
            title,
            author,
            isbn,
            published_year,
            description,
            cover_url,
            pages,
        }
    }

    /// Insert book into the DB
    pub async fn create(&mut self, pool: &MySqlPool) -> Result<(), sqlx::Error> {
        let res = sqlx::query!(
            "INSERT INTO books (title, author, isbn, published_year, description, cover_url, pages) VALUES (?, ?, ?, ?, ?, ?, ?)",
            self.title,
            self.author,
            self.isbn,
            self.published_year,
            self.description,
            self.cover_url,
            self.pages,
        )
        .execute(pool)
        .await?;

        self.id = res.last_insert_id() as i64;

        Ok(())
    }

    /// Update book into the DB. Returns the number of updated rows on Ok.
    pub async fn update(&mut self, pool: &MySqlPool) -> Result<u64, sqlx::Error> {
        let updated = sqlx::query!(
            "UPDATE books SET title = ?, author = ?, isbn = ?, published_year = ?, description = ?, cover_url = ?, pages = ?",
            self.title,
            self.author,
            self.isbn,
            self.published_year,
            self.description,
            self.cover_url,
            self.pages,
        )
        .execute(pool)
        .await?;

        Ok(updated.rows_affected())
    }

    /// Fetches the book with the id from self.
    pub async fn fetch(&self, pool: &MySqlPool) -> Result<Self, sqlx::Error> {
        let record = sqlx::query_as!(
            Book,
            "SELECT id, title, author, isbn, published_year, description, cover_url, pages FROM books WHERE id = ?",
            self.id
        )
        .fetch_one(pool)
        .await?;
        Ok(record)
    }

    /// Fetch a book by its ID.
    pub async fn get(pool: &MySqlPool, id: i64) -> Result<Self, sqlx::Error> {
        let record = sqlx::query_as!(
            Book,
            "SELECT id, title, author, isbn, published_year, description, cover_url, pages  FROM books WHERE id = ?",
            id
        )
        .fetch_one(pool)
        .await?;

        Ok(record)
    }

    /// Deletes the row associated with the record
    pub async fn delete(&self, pool: &MySqlPool) -> Result<u64, sqlx::Error> {
        let deleted = sqlx::query_as!(Book, "DELETE FROM books WHERE id = ?", self.id)
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
        let mut u = Book::new(
            0,
            "Crime and Punishment".to_string(),
            "Fyodor Dostoevsky".to_string(),
            "9780307829603".to_string(),
            2012,
            Some("FYODOR MIKAILOVICH DOSTOEVSKY's life was as dark and dramatic as the great novels he wrote. He was born in Moscow in 1821. A short first novel, Poor Folk (1846) brought him instant success, but his writing career was cut short by his arrest for alleged subversion against Tsar Nicholas I in 1849. In prison he was given the “silent treatment” for eight months (guards even wore velvet soled boots) before he was led in front a firing squad. Dressed in a death shroud, he faced an open grave and awaited execution, when suddenly, an order arrived commuting his sentence. He then spent four years at hard labor in a Siberian prison, where he began to suffer from epilepsy, and he returned to St. Petersburg only a full ten years after he had left in chains.\n His prison experiences coupled with his conversion to a profoundly religious philosophy formed the basis for his great novels. But it was his fortuitous marriage to Anna Snitkina, following a period of utter destitution brought about by his compulsive gambling, that gave Dostoevsky the emotional stability to complete Crime and Punishment (1866), The Idiot (1868-69), The Possessed (1871-72),and The Brothers Karamazov (1879-80). When Dostoevsky died in 1881, he left a legacy of masterworks that influenced the great thinkers and writers of the Western world and immortalized him as a giant among writers of world literature.".to_string()),
            None,
            592,
        );

        // Act
        u.create(&pool).await.unwrap();
        let fetched = Book::get(&pool, u.id).await.unwrap();
        let deleted_rows = u.delete(&pool).await.unwrap();

        // Assert

        // Asserts the db has the correct values
        assert_eq!("Crime and Punishment".to_string(), fetched.title);
        assert_eq!("Fyodor Dostoevsky".to_string(), fetched.author);
        assert_eq!("9780307829603".to_string(), fetched.isbn);
        assert_eq!(2012, fetched.published_year);
        assert_eq!(
            Some("FYODOR MIKAILOVICH DOSTOEVSKY's life was as dark and dramatic as the great novels he wrote. He was born in Moscow in 1821. A short first novel, Poor Folk (1846) brought him instant success, but his writing career was cut short by his arrest for alleged subversion against Tsar Nicholas I in 1849. In prison he was given the “silent treatment” for eight months (guards even wore velvet soled boots) before he was led in front a firing squad. Dressed in a death shroud, he faced an open grave and awaited execution, when suddenly, an order arrived commuting his sentence. He then spent four years at hard labor in a Siberian prison, where he began to suffer from epilepsy, and he returned to St. Petersburg only a full ten years after he had left in chains.\n His prison experiences coupled with his conversion to a profoundly religious philosophy formed the basis for his great novels. But it was his fortuitous marriage to Anna Snitkina, following a period of utter destitution brought about by his compulsive gambling, that gave Dostoevsky the emotional stability to complete Crime and Punishment (1866), The Idiot (1868-69), The Possessed (1871-72),and The Brothers Karamazov (1879-80). When Dostoevsky died in 1881, he left a legacy of masterworks that influenced the great thinkers and writers of the Western world and immortalized him as a giant among writers of world literature.".to_string()), fetched.description
        );
        assert_eq!(None, fetched.cover_url);
        assert_eq!(592, fetched.pages);

        // Asserts the struct has the same values as the db
        assert_eq!(u.id, fetched.id);
        assert_eq!(u.title, fetched.title);
        assert_eq!(u.author, fetched.author);
        assert_eq!(u.isbn, fetched.isbn);
        assert_eq!(u.published_year, fetched.published_year);
        assert_eq!(u.description, fetched.description);
        assert_eq!(u.cover_url, fetched.cover_url);
        assert_eq!(u.pages, fetched.pages);

        // Asserts that one row is deleted (i.e. the one we just set up
        assert_eq!(1, deleted_rows)
    }

    #[tokio::test]
    async fn update() {
        // Arrange
        let pool = setup_db().await;
        let mut u = Book::new(
            0,
            "Crime and Punishment".to_string(),
            "Fyodor Dostoevsky".to_string(),
            "9780307829603".to_string(),
            2012,
            Some("FYODOR MIKAILOVICH DOSTOEVSKY's life was as dark and dramatic as the great novels he wrote. He was born in Moscow in 1821. A short first novel, Poor Folk (1846) brought him instant success, but his writing career was cut short by his arrest for alleged subversion against Tsar Nicholas I in 1849. In prison he was given the “silent treatment” for eight months (guards even wore velvet soled boots) before he was led in front a firing squad. Dressed in a death shroud, he faced an open grave and awaited execution, when suddenly, an order arrived commuting his sentence. He then spent four years at hard labor in a Siberian prison, where he began to suffer from epilepsy, and he returned to St. Petersburg only a full ten years after he had left in chains.\n His prison experiences coupled with his conversion to a profoundly religious philosophy formed the basis for his great novels. But it was his fortuitous marriage to Anna Snitkina, following a period of utter destitution brought about by his compulsive gambling, that gave Dostoevsky the emotional stability to complete Crime and Punishment (1866), The Idiot (1868-69), The Possessed (1871-72),and The Brothers Karamazov (1879-80). When Dostoevsky died in 1881, he left a legacy of masterworks that influenced the great thinkers and writers of the Western world and immortalized him as a giant among writers of world literature.".to_string()),
            None,
            592,
        );

        // Act
        u.create(&pool).await.unwrap();
        let fetched = Book::get(&pool, u.id).await.unwrap();

        u.title = "Brothers Karamazov".to_string();
        u.description = None;

        let updated_rows = u.update(&pool).await.unwrap();

        let updated_fetch = Book::get(&pool, u.id).await.unwrap();

        let deleted_rows = u.delete(&pool).await.unwrap();

        // Assert
        assert_ne!(fetched.title, updated_fetch.title);
        assert_ne!(fetched.description, updated_fetch.description);

        assert_eq!(1, updated_rows);
        assert_eq!(1, deleted_rows);
    }
}
