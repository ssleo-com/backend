use crate::migration::Migration;
use sqlx::PgPool;

const UP_QUERY: &str = "
    CREATE TABLE migrations (
        id SERIAL PRIMARY KEY,
        name VARCHAR(255) NOT NULL,
        applied TIMESTAMP DEFAULT CURRENT_TIMESTAMP
    );
";

const DOWN_QUERY: &str = "DROP TABLE migrations;";

pub struct M0001CreateMigrationsTable;

impl Migration for M0001CreateMigrationsTable {
    async fn up(&self, pool: &PgPool) -> Result<(), sqlx::Error> {
        sqlx::query(UP_QUERY).execute(pool).await?;
        Ok(())
    }

    async fn down(&self, pool: &PgPool) -> Result<(), sqlx::Error> {
        sqlx::query(DOWN_QUERY).execute(pool).await?;
        Ok(())
    }
}
