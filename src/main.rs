use std::sync::Arc;

use async_trait::async_trait;
use sqlx::Sqlite;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    start().await
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

async fn start() -> Result<(), Box<dyn std::error::Error>> {
    let pool = sqlx::SqlitePool::connect("sqlite:sandbox.db").await?;
    let state = State {
        repo: Arc::new(SqliteRepository { pool }),
    };
    let users = state.repo.list().await;
    println!("{:?}", users);
    return Ok(());
}
