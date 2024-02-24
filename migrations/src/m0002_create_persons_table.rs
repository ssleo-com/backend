use crate::migrations::Migration;
use sqlx::PgPool;

const UP_QUERY: &str = "
    CREATE TABLE persons (
        id SERIAL PRIMARY KEY,
        first_name VARCHAR(255) NOT NULL,
        last_name VARCHAR(255) NOT NULL,
        date_of_birth DATE,
        created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
        updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
    );
";

const DOWN_QUERY: &str = "DROP TABLE persons;";

pub struct M0002CreatePersonsTable;

impl Migration for M0002CreatePersonsTable {
    async fn up(&self, pool: &PgPool) -> Result<(), sqlx::Error> {
        sqlx::query(UP_QUERY).execute(pool).await?;
        Ok(())
    }

    async fn down(&self, pool: &PgPool) -> Result<(), sqlx::Error> {
        sqlx::query(DOWN_QUERY).execute(pool).await?;
        Ok(())
    }
}
