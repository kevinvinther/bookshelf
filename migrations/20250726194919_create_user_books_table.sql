-- Add migration script here
CREATE TABLE user_books (
    user_id BIGINT NOT NULL,
    book_id BIGINT NOT NULL,
    status ENUM('to-read', 'reading', 'completed') DEFAULT 'to-read' NOT NULL,
    rating TINYINT UNSIGNED CHECK (rating >= 0 AND rating <= 10),
    added_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
    began_reading TIMESTAMP,
    done_reading TIMESTAMP,
    current_page INT UNSIGNED,
    PRIMARY KEY (user_id, book_id),
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
    FOREIGN KEY (book_id) REFERENCES books(id) ON DELETE CASCADE
);
