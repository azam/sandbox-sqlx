use sqlx::Sqlite;

#[derive(Debug, sqlx::FromRow)]
struct User {
    id: i64,
    name: String,
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let pool = sqlx::SqlitePool::connect("sqlite:sandbox.db").await?;
    let users: Vec<User> = sqlx::query_as::<Sqlite, User>("SELECT * FROM \"users\"")
        .fetch_all(&pool)
        .await?;
    println!("{:?}", users);
    return Ok(());
}