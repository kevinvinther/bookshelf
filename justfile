DATABASE_URL := "postgres://dev:dev@localhost:5432/bookshelf"
TEST_DATABASE_URL := "postgres://dev:dev@localhost:5432/bookshelf_test"

# Drops and recreates the database
reset-db:
    sqlx database drop --database-url {{DATABASE_URL}}
    sqlx database create --database-url {{DATABASE_URL}}
    sqlx migrate run --database-url {{DATABASE_URL}}
    cargo sqlx prepare

# Drops and recreates the test database
reset-test-db:
    sqlx database drop --database-url {{TEST_DATABASE_URL}}
    sqlx database create --database-url {{TEST_DATABASE_URL}}
    sqlx migrate run --database-url {{TEST_DATABASE_URL}}
    cargo sqlx prepare

# Restarts the database containe
restart-db:
    docker compose down
    docker compose up -d --build --remove-orphans

# Just run migrations and prepare (if you haven't changed them)
reload:
    sqlx migrate run --database-url {{DATABASE_URL}}
    cargo sqlx prepare

# Deletes .sqlx cache and prepares fresh (optional)
prepare-clean:
    rm -rf .sqlx
    cargo sqlx prepare
