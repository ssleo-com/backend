use crate::get_env_var;
use sqlx::PgPool;

fn get_pg_url(
    pg_database: &String,
    pg_host: &String,
    pg_pass: &String,
    pg_port: &String,
    pg_user: &String,
) -> String {
    format!(
        "postgres://{}:{}@{}:{}/{}",
        pg_user, pg_pass, pg_host, pg_port, pg_database
    )
}

pub async fn get_pg_pool() -> PgPool {
    let pg_database = get_env_var("PG_DATABASE");
    let pg_host = get_env_var("PG_HOST");

    let pg_pass = get_env_var("PG_PASSWORD");
    let pg_port = get_env_var("PG_PORT");
    let pg_user = get_env_var("PG_USER");

    let url = get_pg_url(&pg_database, &pg_host, &pg_pass, &pg_port, &pg_user);

    println!("Connecting to {}", url);
    let pool = sqlx::postgres::PgPool::connect(&url).await.unwrap();

    pool
}
