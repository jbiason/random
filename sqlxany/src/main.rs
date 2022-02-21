use sqlx::{Any, Pool};

mod entity;

#[tokio::main]
async fn main() {
    let record = entity::Entity::new(1, "Something");
    let pool =
        Pool::<Any>::connect(&std::env::var("DATABASE_URL").expect("I need DATABASE_URL set!"))
            .await
            .expect("Failed to connect to the database");
    record.save(&pool).await.unwrap();
}
