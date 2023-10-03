use std::sync::Arc;

use async_trait::async_trait;
use sqlx::{database::Database, postgres::Postgres, sqlite::Sqlite};

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
        DatabaseTypes::Sqlite => Arc::new(Repo::<Sqlite>::new(
            std::env::var("SANDBOX_SQLITE_URL")?.as_str(),
        )),
        DatabaseTypes::Postgres => Arc::new(Repo::<Postgres>::new(
            std::env::var("SANDBOX_POSTGRES_URL")?.as_str(),
        )),
    };
    let state = State { repo };
    println!("list(): {:?}", state.repo.list().await);
    println!("get(1): {:?}", state.repo.get(1).await);
    println!("get(7): {:?}", state.repo.get(7).await);
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
    async fn get(&self, id: i64) -> Option<User>;
}

#[derive(Debug, Clone)]
struct State {
    repo: Arc<dyn Repository>, // Cannot use box because it requires Clone trait
}

#[derive(Debug)]
struct Repo<DB>
where
    DB: Database,
{
    pool: sqlx::pool::Pool<DB>,
}

/**
 * Common functions, generic to all database
 */
impl<DB> Repo<DB> where DB: Database {
    pub fn new(url: &str) -> Repo<DB> {
        Self {
            pool: sqlx::pool::Pool::connect_lazy(url).unwrap(),
        }
    }
}

#[async_trait]
impl Repository for Repo<Sqlite> {
    async fn list(&self) -> Vec<User> {
        sqlx::query_as("SELECT * FROM \"users\"")
            .fetch_all(&self.pool)
            .await
            .unwrap()
    }

    async fn get(&self, id: i64) -> Option<User> {
        sqlx::query_as("SELECT * FROM \"users\" WHERE id = ?")
            .bind(id)
            .fetch_optional(&self.pool)
            .await
            .unwrap()
    }
}

#[async_trait]
impl Repository for Repo<Postgres> {
    async fn list(&self) -> Vec<User> {
        sqlx::query_as("SELECT * FROM \"users\"")
            .fetch_all(&self.pool)
            .await
            .unwrap()
    }

    async fn get(&self, id: i64) -> Option<User> {
        sqlx::query_as("SELECT * FROM \"users\" WHERE id = $1")
            .bind(id)
            .fetch_optional(&self.pool)
            .await
            .unwrap()
    }
}
