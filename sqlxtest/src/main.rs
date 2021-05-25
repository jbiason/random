use sqlx::pool::Pool;
use sqlx::sqlite::Sqlite;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let pool = Pool::<Sqlite>::connect("sqlite::memory:").await?;
    sqlx::migrate!("db/migrations").run(&pool).await?;
    Ok(())
}
