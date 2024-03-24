use sqlx::PgPool;
use std::future::Future;

use crate::models::Person;

pub trait Delete {
    fn delete(&self, pool: &PgPool) -> impl Future<Output = Result<(), sqlx::Error>>;
}

impl Delete for Person {
    fn delete(&self, pool: &PgPool) -> impl Future<Output = Result<(), sqlx::Error>> {
        async move {
            sqlx::query(
                r#"
                DELETE FROM persons
                WHERE id = $1
                "#,
            )
            .bind(&self.id)
            .execute(pool)
            .await
            .expect("Failed to delete person from the database");

            Ok(())
        }
    }
}
