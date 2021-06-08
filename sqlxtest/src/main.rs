use sqlx::sqlite::Sqlite;
use sqlx::sqlite::SqliteConnectOptions;
use sqlx::sqlite::SqlitePoolOptions;
use sqlx::Pool;
use std::env;

struct Label {
    id: Option<u32>,
    description: String,
}

struct LabelRepository<'a> {
    pool: &'a Pool<Sqlite>,
}

impl<'a> LabelRepository<'a> {
    fn new(pool: &'a Pool<Sqlite>) -> Self {
        Self { pool }
    }

    async fn save(&self, label: &Label) -> Result<(), sqlx::Error> {
        sqlx::query(r#"INSERT INTO testing (label) VALUES (?)"#)
            .bind(&label.description)
            .execute(self.pool)
            .await?;
        Ok(())
    }

    async fn find_by_description(&self, label: &str) -> Result<Label, sqlx::Error> {
        let result: (u32, String) =
            sqlx::query_as(r#"SELECT id, label FROM testing WHERE label = ?"#)
                .bind(label)
                .fetch_one(self.pool)
                .await?;
        Ok(Label {
            id: Some(result.0),
            description: result.1,
        })
    }

    async fn delete(&self, record: &Label) -> Result<(), sqlx::Error> {
        sqlx::query(r#"DELETE FROM testing WHERE id = ?"#)
            .bind(&record.id)
            .execute(self.pool)
            .await?;
        Ok(())
    }
}

async fn connect() -> Result<Pool<Sqlite>, sqlx::Error> {
    println!("Open database");
    let pool = SqlitePoolOptions::new()
        .connect_with(
            SqliteConnectOptions::new()
                .filename("testing.sqlite")
                .create_if_missing(true),
        )
        .await?;
    sqlx::migrate!("db/migrations").run(&pool).await?;
    Ok(pool)
}

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let pool = connect().await?;
    let repo = LabelRepository::new(&pool);

    let command = env::args()
        .nth(1)
        .expect("Need a command: \"add\" or \"remove\"");
    let value = env::args()
        .nth(2)
        .expect("Besides the command, need a label to deal with");

    println!("Command: \"{}\"", command);

    if command == "add" {
        println!("Should add \"{}\"", value);
        let record = Label {
            id: None,
            description: value.into(),
        };
        repo.save(&record).await?;
    } else if command == "remove" {
        println!("Should remove \"{}\"", value);
        if let Ok(record) = repo.find_by_description(&value).await {
            repo.delete(&record).await?;
            println!("Removed");
        } else {
            println!("Label does not exist");
        }
    }
    Ok(())
}
