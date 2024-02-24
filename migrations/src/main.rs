mod m0001_create_migrations_table;
mod m0002_create_persons_table;

use dotenv::dotenv;
use m0001_create_migrations_table::M0001CreateMigrationsTable;
use m0002_create_persons_table::M0002CreatePersonsTable;
use shared::{does_table_exist, get_pg_pool::get_pg_pool};
mod migrations;
use sqlx::Row;

use migrations::{Migration, Migrations};

async fn get_last_migration(pool: &sqlx::Pool<sqlx::Postgres>) -> Result<Option<i32>, sqlx::Error> {
    if let Ok(Some(id)) = sqlx::query("SELECT MAX(id) FROM migrations")
        .fetch_optional(pool)
        .await
    {
        let last_migration: Option<i32> = id.get("MAX(id)");

        println!("Last migration: {:?}", last_migration);
        return Ok(last_migration);
    }

    Err(sqlx::Error::RowNotFound)
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

    if migrations_table_exists {
        let last_migration: Option<i32> = get_last_migration(&pool).await.unwrap();
    }

    let migrations = vec![
        Migrations::M0001(M0001CreateMigrationsTable),
        Migrations::M0002(M0002CreatePersonsTable),
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
