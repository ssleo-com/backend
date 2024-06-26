use crate::get_env_var;
use sqlx::PgPool;

fn get_pg_url(
    pg_user: &String,
    pg_pass: &String,
    pg_host: &String,
    pg_port: &String,
    pg_database: &Option<String>,
) -> String {
    match pg_database {
        Some(db) => {
            format!(
                "postgres://{}:{}@{}:{}/{}",
                pg_user, pg_pass, pg_host, pg_port, db
            )
        }
        None => {
            format!(
                "postgres://{}:{}@{}:{}/",
                pg_user, pg_pass, pg_host, pg_port
            )
        }
    }
}

pub async fn get_pg_pool(pg_database: &Option<String>) -> PgPool {
    let pg_host = get_env_var("PG_HOST");
    let pg_pass = get_env_var("PG_PASSWORD");
    let pg_port = get_env_var("PG_PORT");
    let pg_user = get_env_var("PG_USER");

    let url = get_pg_url(&pg_user, &pg_pass, &pg_host, &pg_port, &pg_database);

    let pool = sqlx::postgres::PgPool::connect(&url).await.unwrap();

    pool
}
