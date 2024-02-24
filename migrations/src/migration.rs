use sqlx::PgPool;

use crate::{
    m0001_create_migrations_table::M0001CreateMigrationsTable,
    m0002_create_persons_table::M0002CreatePersonsTable,
};

pub enum AllMigrations {
    M0001(M0001CreateMigrationsTable),
    M0002(M0002CreatePersonsTable),
}

pub trait Migration {
    async fn up(&self, pool: &PgPool) -> Result<(), sqlx::Error>;
    async fn down(&self, pool: &PgPool) -> Result<(), sqlx::Error>;
}
