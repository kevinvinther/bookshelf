CREATE TYPE reading_status AS ENUM ('to-read', 'reading', 'completed');

CREATE TABLE user_books (
    user_id BIGINT NOT NULL,
    book_id BIGINT NOT NULL,
    status reading_status DEFAULT 'to-read' NOT NULL,
    rating SMALLINT CHECK (rating >= 0 AND rating <= 10),
    added_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL,
    began_reading TIMESTAMPTZ,
    done_reading TIMESTAMPTZ,
    current_page INTEGER CHECK (current_page >= 0),
    PRIMARY KEY (user_id, book_id),
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
    FOREIGN KEY (book_id) REFERENCES books(id) ON DELETE CASCADE
);

CREATE INDEX idx_user_books_status ON user_books(status);
CREATE INDEX idx_user_books_current_page ON user_books(current_page);
