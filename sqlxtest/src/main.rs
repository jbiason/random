use sqlx::sqlite::SqliteConnectOptions;
use sqlx::sqlite::SqlitePoolOptions;
use sqlx::Pool;
use std::env;

struct Label {
    id: u32,
    description: String,
}

struct Repository<T>
where
    T: sqlx::Database,
{
    pool: Pool<T>,
}

impl<T> Repository<T>
where
    T: sqlx::Database,
{
    fn new(pool: Pool<T>) -> Self {
        Self { pool }
    }

    async fn save(&mut self, label: &Label) -> Result<(), sqlx::Error> {
        sqlx::query(r#"INSERT INTO testing (label) VALUES (?)"#)
            .bind(label.description)
            .execute(self.pool)
            .await?;
        Ok(())
    }
}

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

    let repo = Repository::new(pool);

    let command = env::args().nth(1).unwrap();
    println!("Command: \"{}\"", command);
    if command == "add" {
        let value = env::args().nth(2).unwrap();
        println!("Should add \"{}\"", value);
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
