DATABASE_URL := "mysql://dev:dev@localhost:3306/bookshelf"

# Drops and recreates the database
reset-db:
    sqlx database drop --database-url {{DATABASE_URL}}
    sqlx database create --database-url {{DATABASE_URL}}
    sqlx migrate run --database-url {{DATABASE_URL}}
    cargo sqlx prepare

# Just run migrations and prepare (if you haven't changed them)
reload:
    sqlx migrate run --database-url {{DATABASE_URL}}
    cargo sqlx prepare

# Deletes .sqlx cache and prepares fresh (optional)
prepare-clean:
    rm -rf .sqlx
    cargo sqlx prepare
