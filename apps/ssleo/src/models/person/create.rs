use sqlx::PgPool;

use structs::{NewPerson, Person};

pub trait Create {
    fn create(
        &self,
        pool: &PgPool,
    ) -> impl std::future::Future<Output = Result<Person, sqlx::Error>> + Send;
}

impl Create for NewPerson {
    async fn create(&self, pool: &PgPool) -> Result<Person, sqlx::Error> {
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
