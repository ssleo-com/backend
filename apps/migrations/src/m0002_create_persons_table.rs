use crate::migrations::MigrationHandlers;
use sqlx::PgPool;

const UP_QUERY: &str = "
    CREATE TABLE persons (
        id SERIAL PRIMARY KEY,
        first_name VARCHAR(255) NOT NULL,
        last_name VARCHAR(255) NOT NULL,
        date_of_birth DATE,
        created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
        updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
    );
";

const DOWN_QUERY: &str = "DROP TABLE persons;";

const INSERT_MIGRATION: &str =
    "INSERT INTO migrations (id, name) VALUES (2, 'M0002CreatePersonsTable');";

const REMOVE_MIGRATION: &str = "DELETE FROM migrations WHERE name = 'M0002CreatePersonsTable'";

#[derive(Debug)]
pub struct M0002CreatePersonsTable;

impl MigrationHandlers for M0002CreatePersonsTable {
    async fn up(&self, pool: &PgPool) -> Result<(), sqlx::Error> {
        sqlx::query(UP_QUERY).execute(pool).await?;
        sqlx::query(INSERT_MIGRATION).execute(pool).await?;
        Ok(())
    }

    async fn down(&self, pool: &PgPool) -> Result<(), sqlx::Error> {
        sqlx::query(DOWN_QUERY).execute(pool).await?;
        sqlx::query(REMOVE_MIGRATION).execute(pool).await?;
        Ok(())
    }
}
