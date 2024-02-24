use crate::migrations::MigrationHandlers;
use sqlx::PgPool;

const UP_QUERY: &str = "
    CREATE TABLE migrations (
        id SERIAL PRIMARY KEY,
        name VARCHAR(255) NOT NULL,
        applied TIMESTAMP DEFAULT CURRENT_TIMESTAMP
    );
";

const DOWN_QUERY: &str = "DROP TABLE migrations;";

const INSERT_MIGRATION: &str =
    "INSERT INTO migrations (name) VALUES ('M0001CreateMigrationsTable');";

#[derive(Debug)]
pub struct M0001CreateMigrationsTable;

impl MigrationHandlers for M0001CreateMigrationsTable {
    async fn up(&self, pool: &PgPool) -> Result<(), sqlx::Error> {
        sqlx::query(UP_QUERY).execute(pool).await?;
        sqlx::query(INSERT_MIGRATION).execute(pool).await?;
        Ok(())
    }

    async fn down(&self, pool: &PgPool) -> Result<(), sqlx::Error> {
        sqlx::query(DOWN_QUERY).execute(pool).await?;
        Ok(())
    }
}
