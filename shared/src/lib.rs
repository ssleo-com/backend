use sqlx::PgPool;
use std::env;
pub mod get_pg_pool;

pub async fn does_table_exist(name: &str, pool: &PgPool) -> bool {
    let query = format!(
        "
            SELECT EXISTS (
                SELECT 1
                FROM   information_schema.tables
                WHERE  table_catalog = 'ssleo'
                AND    table_name = '{}'
            );
        ",
        name
    );

    let exists: bool = sqlx::query_scalar(&query).fetch_one(pool).await.unwrap();

    exists
}

pub fn get_env_var(name: &str) -> String {
    match env::var(name) {
        Ok(val) => val,
        Err(_) => {
            println!(
                "{} environment variable is missing. Please add it to your .env file.",
                name
            );
            std::process::exit(1);
        }
    }
}
