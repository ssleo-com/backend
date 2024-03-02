use crate::Person;
use sqlx::PgPool;

pub trait Update {
    // TODO: Fix below warning
    async fn update(&self, pool: &PgPool) -> Result<Person, sqlx::Error>;
}

impl Update for Person {
    async fn update(&self, pool: &PgPool) -> Result<Person, sqlx::Error> {
        let row = sqlx::query_as!(
            Person,
            r#"
            INSERT INTO persons (first_name, last_name, date_of_birth)
            VALUES ($1, $2, $3)
            RETURNING id, first_name, last_name, date_of_birth, created_at, updated_at
            "#,
            self.first_name,
            self.last_name,
            self.date_of_birth
        )
        .fetch_one(pool)
        .await?;

        Ok(row)
    }
}
