// migration_runner.rs

mod m0001_create_migrations_table;
mod m0002_create_persons_table;

use dotenv::dotenv;
use m0001_create_migrations_table::M0001CreateMigrationsTable;
use m0002_create_persons_table::M0002CreatePersonsTable;
use shared::get_pg_pool::get_pg_pool;
use sqlx::PgPool;
mod migration;

use migration::{AllMigrations, Migration};

impl Migration for AllMigrations {
    async fn up(&self, pool: &PgPool) -> Result<(), sqlx::Error> {
        match self {
            AllMigrations::M0001(migration) => migration.up(pool).await,
            AllMigrations::M0002(migration) => migration.up(pool).await,
            // Add other cases as needed
        }
    }

    async fn down(&self, pool: &PgPool) -> Result<(), sqlx::Error> {
        match self {
            AllMigrations::M0001(migration) => migration.down(pool).await,
            AllMigrations::M0002(migration) => migration.down(pool).await,
            // Add other cases as needed
        }
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let pool = get_pg_pool().await;

    let args: Vec<String> = std::env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: {} <migration_number>", args[0]);
        std::process::exit(1);
    }

    let target_migration: i32 = args[1].parse().expect("Invalid migration number");

    let migrations = vec![
        AllMigrations::M0001(M0001CreateMigrationsTable),
        AllMigrations::M0002(M0002CreatePersonsTable),
    ];

    for (i, migration) in migrations.iter().enumerate() {
        if i as i32 <= target_migration {
            match migration.up(&pool).await {
                Ok(_) => println!("Migration {} applied successfully", i),
                Err(err) => {
                    eprintln!("Failed to apply migration {}: {}", i, err);
                    std::process::exit(1);
                }
            }
        } else {
            match migration.down(&pool).await {
                Ok(_) => println!("Rollback of migration {} successful", i),
                Err(err) => {
                    eprintln!("Failed to rollback migration {}: {}", i, err);
                    std::process::exit(1);
                }
            }
        }
    }
}
