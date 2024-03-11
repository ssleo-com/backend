mod m0001_create_migrations_table;
mod m0002_create_persons_table;

use dotenv::dotenv;
use m0001_create_migrations_table::M0001CreateMigrationsTable;
use m0002_create_persons_table::M0002CreatePersonsTable;
use shared::{
    does_table_exist,
    get_pg_pool::{conn, test_conn},
};
mod migrations;
use sqlx::{postgres::PgRow, PgPool, Row};

use migrations::{MigrationHandlers, Migrations};
use std::cmp::Ordering;

async fn get_last_migration(pool: &sqlx::Pool<sqlx::Postgres>) -> PgRow {
    let max_query = sqlx::query("SELECT id, name FROM migrations ORDER BY id DESC LIMIT 1")
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

async fn apply_migration(
    pool: &sqlx::Pool<sqlx::Postgres>,
    migration: &Migrations,
    direction: &str,
) {
    match direction {
        "up" => match migration.up(pool).await {
            Ok(_) => println!("Migration {:?} applied successfully", migration),
            Err(err) => {
                eprintln!("Failed to apply migration {:?}: {}", migration, err);
                std::process::exit(1);
            }
        },
        "down" => match migration.down(pool).await {
            Ok(_) => println!("Migration {:?} rolled back successfully", migration),
            Err(err) => {
                eprintln!("Failed to roll back migration {:?}: {}", migration, err);
                std::process::exit(1);
            }
        },
        _ => {
            eprintln!("Invalid migration direction: {}", direction);
            std::process::exit(1);
        }
    }
}

async fn get_migration_by_number(number: i32) -> Option<Migrations> {
    match number {
        1 => Some(Migrations::M0001(M0001CreateMigrationsTable)),
        2 => Some(Migrations::M0002(M0002CreatePersonsTable)),
        _ => None,
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let args: Vec<String> = std::env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: {} <migration_number>", args[0]);
        std::process::exit(1);
    }

    let pool: PgPool = test_conn().await;

    let target_migration: i32 = args[1].parse().expect("Invalid migration number");
    let migrations_table_exists: bool = does_table_exist("migrations", &pool).await;

    let mut last_migration_number: i32 = 0;

    if migrations_table_exists {
        let last_migration: PgRow = get_last_migration(&pool).await;
        last_migration_number = last_migration.get("id");
    }

    match target_migration.cmp(&last_migration_number) {
        Ordering::Equal => {
            println!("Desired migration is already applied. No action needed.");
        }
        Ordering::Greater => {
            for i in (last_migration_number + 1)..=target_migration {
                match get_migration_by_number(i).await {
                    Some(migration) => {
                        apply_migration(&pool, &migration, "up").await;
                    }
                    None => {
                        eprintln!("Migration {} not found.", i);
                        std::process::exit(1);
                    }
                }
            }
        }
        Ordering::Less => {
            for i in ((target_migration + 1)..=last_migration_number).rev() {
                match get_migration_by_number(i).await {
                    Some(migration) => {
                        apply_migration(&pool, &migration, "down").await;
                    }
                    None => {
                        eprintln!("Migration {} not found.", i);
                        std::process::exit(1);
                    }
                }
            }
        }
    }
}
