use sqlx::PgPool;

use crate::{
    m0001_create_migrations_table::M0001CreateMigrationsTable,
    m0002_create_persons_table::M0002CreatePersonsTable,
};

#[derive(Debug)]
pub enum Migrations {
    M0001(M0001CreateMigrationsTable),
    M0002(M0002CreatePersonsTable),
}

pub trait MigrationHandlers {
    async fn up(&self, pool: &PgPool) -> Result<(), sqlx::Error>;
    async fn down(&self, pool: &PgPool) -> Result<(), sqlx::Error>;
}

impl MigrationHandlers for Migrations {
    async fn up(&self, pool: &PgPool) -> Result<(), sqlx::Error> {
        match self {
            Migrations::M0001(migration) => migration.up(pool).await,
            Migrations::M0002(migration) => migration.up(pool).await,
        }
    }

    async fn down(&self, pool: &PgPool) -> Result<(), sqlx::Error> {
        match self {
            Migrations::M0001(migration) => migration.down(pool).await,
            Migrations::M0002(migration) => migration.down(pool).await,
        }
    }
}
