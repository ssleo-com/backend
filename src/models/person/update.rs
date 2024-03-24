use crate::models::Person;
use sqlx::PgPool;

pub trait Update {
    fn update(
        &self,
        pool: &PgPool,
    ) -> impl std::future::Future<Output = Result<Person, sqlx::Error>> + Send;
}

impl Update for Person {
    async fn update(&self, pool: &PgPool) -> Result<Person, sqlx::Error> {
        let row = sqlx::query_as!(
            Person,
            r#"
            UPDATE persons SET first_name = $1, last_name = $2, date_of_birth = $3
            WHERE id = $4
            RETURNING id, first_name, last_name, date_of_birth, created_at, updated_at
            "#,
            self.first_name,
            self.last_name,
            self.date_of_birth,
            self.id
        )
        .fetch_one(pool)
        .await?;

        Ok(row)
    }
}
