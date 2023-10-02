# sandbox-sqlx

Sample code to implement a repository layer on multiple database implementations.

## Configuration

sqlx compile time query check is disabled because we are developing queries for different databases. This is done by setting `SQLX_OFFLINE=true` env var, in our case it is set [here](.cargo/config.toml), along with other env vars.

## Prepare

### sqlx

1. `cargo install sqlx-cli`

### Postgres

1. `docker run --name sandbox-sqlx-postgres -p 5000:5432 -e POSTGRES_PASSWORD="sandbox" -d postgres`
2. `sqlx database create -D postgres://postgres:sandbox@localhost:5000/sandbox`
3. `sqlx migrate run --source migrations/postgres -D postgres://postgres:sandbox@localhost:5000/sandbox`

### Sqlite

1. `sqlx database create -D sqlite:sandbox-sqlx-sqlite.db`
2. `sqlx migrate run --source migrations/sqlite -D sqlite:sandbox-sqlx-sqlite.db`

## Execute

### Sqlite

1. `cargo run -- sqlite`

### Postgres

1. `cargo run -- postgres`

## Cleanup

### Postgres

1. `sqlx database drop -D postgres://postgres:sandbox@localhost:5000/sandbox`
2. `docker stop sandbox-sqlx-postgres`
3. `docker rm sandbox-sqlx-postgres`

### Sqlite

1. `rm sandbox-sqlx-sqlite.db`