use sqlx::sqlite::SqliteConnectOptions;
use sqlx::sqlite::SqlitePoolOptions;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    println!("Open");
    let pool = SqlitePoolOptions::new()
        .connect_with(
            SqliteConnectOptions::new()
                .filename("testing.sqlite")
                .create_if_missing(true),
        )
        .await?;
    println!("Migrate");
    sqlx::migrate!("db/migrations").run(&pool).await?;
    println!("Done");
    Ok(())
}
