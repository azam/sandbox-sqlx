use std::sync::Arc;

use async_trait::async_trait;
use sqlx::{Database, Postgres, Sqlite};

enum DatabaseTypes {
    Sqlite,
    Postgres,
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let arg = std::env::args().nth(1).ok_or("invalid args".to_owned())?;
    let db = match arg.as_str() {
        "sqlite" => DatabaseTypes::Sqlite,
        "postgres" => DatabaseTypes::Postgres,
        _ => return Err("invalid args".into()),
    };
    let repo: Arc<dyn Repository> = match db {
        DatabaseTypes::Sqlite => Arc::new(SqliteRepository {
            pool: sqlx::SqlitePool::connect(std::env::var("SANDBOX_SQLITE_URL")?.as_str()).await?,
        }),
        DatabaseTypes::Postgres => Arc::new(PostgresRepository {
            pool: sqlx::PgPool::connect(std::env::var("SANDBOX_POSTGRES_URL")?.as_str()).await?,
        }),
    };
    let state = State { repo };
    let users = state.repo.list().await;
    println!("{:?}", users);
    return Ok(());
}

#[derive(Debug, sqlx::FromRow)]
struct User {
    id: i64,
    name: String,
}

#[async_trait]
trait Repository: std::fmt::Debug {
    async fn list(&self) -> Vec<User>;
}

#[derive(Debug, Clone)]
struct State {
    repo: Arc<dyn Repository>, // Cannot use box because it requires Clone trait
}

#[derive(Debug, Clone)]
struct SqliteRepository {
    pool: sqlx::SqlitePool,
}

#[async_trait]
impl Repository for SqliteRepository {
    async fn list(&self) -> Vec<User> {
        let users = sqlx::query_as::<Sqlite, User>("SELECT * FROM \"users\"")
            .fetch_all(&self.pool)
            .await
            .unwrap();
        return users;
    }
}

#[derive(Debug, Clone)]
struct PostgresRepository {
    pool: sqlx::PgPool,
}

#[async_trait]
impl Repository for PostgresRepository {
    async fn list(&self) -> Vec<User> {
        let users = sqlx::query_as::<Postgres, User>("SELECT * FROM \"users\"")
            .fetch_all(&self.pool)
            .await
            .unwrap();
        return users;
    }
}
