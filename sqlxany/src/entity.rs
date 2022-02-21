//! Uma entidade

use sqlx::{Any, Pool};

const INSERT: &str = r#"
INSERT INTO entity
    (id, description)
VALUES
    ($1, $2);
"#;

#[derive(sqlx::FromRow)]
pub struct Entity {
    id: i64,
    description: String,
}

impl Entity {
    pub fn new(id: i64, description: &str) -> Self {
        Self {
            id,
            description: description.into(),
        }
    }

    pub async fn save(&self, db: &Pool<Any>) -> Result<(), sqlx::Error> {
        sqlx::query(INSERT)
            .bind(&self.id)
            .bind(&self.description)
            .execute(db)
            .await?;
        Ok(())
    }
}
