mod m0001_create_migrations_table;
mod m0002_create_persons_table;

use dotenv::dotenv;
use m0001_create_migrations_table::M0001CreateMigrationsTable;
use m0002_create_persons_table::M0002CreatePersonsTable;
use shared::{does_table_exist, get_pg_pool::get_pg_pool};
mod migrations;
use sqlx::{postgres::PgRow, Row};

use migrations::{MigrationHandlers, Migrations};
use models::Migration;

async fn get_last_migration(pool: &sqlx::Pool<sqlx::Postgres>) -> PgRow {
    let max_query = sqlx::query("SELECT MAX(id), name FROM migrations GROUP BY name")
        .fetch_one(pool)
        .await;

    match max_query {
        Ok(row) => {
            return row;
        }
        Err(err) => {
            eprintln!("Failed to get last migration: {}", err);
            std::process::exit(1)
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

    let migrations_table_exists = does_table_exist("migrations", &pool).await;

    let migrations = vec![
        Migrations::M0001(M0001CreateMigrationsTable),
        Migrations::M0002(M0002CreatePersonsTable),
    ];

    if migrations_table_exists {
        let last_migration: PgRow = get_last_migration(&pool).await;
        println!(
            "Last migration: {:?}",
            last_migration.get::<String, _>("name")
        );
    } else {
        for (i, migration) in migrations.iter().enumerate() {
            match migration.up(&pool).await {
                Ok(_) => println!("Migration {} applied successfully", i),
                Err(err) => {
                    eprintln!("Failed to apply migration {}: {}", i, err);
                    std::process::exit(1);
                }
            }
        }
    }
}
