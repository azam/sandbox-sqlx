use std::sync::Arc;

use async_trait::async_trait;
use sqlx::Sqlite;

enum DatabaseTypes {
    Sqlite,
    Postgres,
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    println!("{:?}", &args);
    let pool = sqlx::SqlitePool::connect(std::env::var("SANDBOX_SQLITE_URL")?.as_str()).await?;
    let state = State {
        repo: Arc::new(SqliteRepository { pool }),
    };
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
