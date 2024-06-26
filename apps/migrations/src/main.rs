mod m0001_create_migrations_table;
mod m0002_create_persons_table;

use dotenv::dotenv;
use m0001_create_migrations_table::M0001CreateMigrationsTable;
use m0002_create_persons_table::M0002CreatePersonsTable;
mod shared;
use shared::{
    does_table_exist::does_table_exist, get_env_var::get_env_var, get_pg_pool::get_pg_pool,
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

pub async fn create_db(db_name: &str, user: &str, pool: &sqlx::PgPool) {
    let query = format!("CREATE DATABASE \"{}\" OWNER \"{}\";", db_name, user);

    match sqlx::query(&query).execute(pool).await {
        Ok(_) => println!("Database created successfully"),
        Err(e) => println!("Error creating database: {}", e),
    }
}

pub async fn db_exists(db_name: &String, pool: &sqlx::PgPool) -> Option<PgRow> {
    let query = r#"SELECT datname FROM pg_database WHERE datname = $1;"#;

    let row: Option<PgRow> = sqlx::query(query)
        .bind(db_name)
        .fetch_optional(pool)
        .await
        .unwrap();

    row
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let args: Vec<String> = std::env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: {} <migration_number>", args[0]);
        std::process::exit(1);
    }

    let db_name = get_env_var("PG_DATABASE");

    let pool: PgPool = get_pg_pool(&None).await;

    let db_exists: Option<PgRow> = db_exists(&db_name, &pool).await;

    if db_exists.is_none() {
        create_db(&db_name, &get_env_var("PG_USER"), &pool).await;
    }

    let pool: PgPool = get_pg_pool(&Some(db_name)).await;

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
