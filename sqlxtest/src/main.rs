use sqlx::sqlite::SqliteConnectOptions;
use sqlx::sqlite::SqlitePoolOptions;
use std::env;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    println!("Open database");
    let pool = SqlitePoolOptions::new()
        .connect_with(
            SqliteConnectOptions::new()
                .filename("testing.sqlite")
                .create_if_missing(true),
        )
        .await?;
    sqlx::migrate!("db/migrations").run(&pool).await?;

    let command = env::args().nth(1).unwrap();
    println!("Command: \"{}\"", command);
    if command == "add" {
        let value = env::args().nth(2).unwrap();
        println!("Should add \"{}\"", value);
        sqlx::query(r#"INSERT INTO testing (label) VALUES (?)"#)
            .bind(value)
            .execute(&pool)
            .await?;
    } else if command == "remove" {
        let value = env::args().nth(2).unwrap();
        println!("Should remove \"{}\"", value);
        sqlx::query(r#"DELETE FROM testing WHERE label = ?"#)
            .bind(value)
            .execute(&pool)
            .await?;
    }
    Ok(())
}
