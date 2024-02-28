use chrono::{NaiveDate, NaiveDateTime};
use serde::Serialize;
use sqlx::PgPool;

#[derive(Serialize)]
pub struct Person {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub date_of_birth: Option<NaiveDate>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Serialize)]

pub struct NewPerson {
    pub first_name: String,
    pub last_name: String,
    pub date_of_birth: Option<NaiveDate>,
}

pub trait Save {
    // TODO: Fix below warning
    async fn save(&self, pool: &PgPool) -> Result<Person, sqlx::Error>;
}

impl Save for NewPerson {
    async fn save(&self, pool: &PgPool) -> Result<Person, sqlx::Error> {
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
