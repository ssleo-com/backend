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

pub async fn conn() -> PgPool {
    let db_name = get_env_var("PG_DATABASE");
    let pool = get_pg_pool(&Some(db_name)).await;
    pool
}

// TODO: Might want to refactor this into an OOP style, with a struct that has a method
// to create the pool, create databases, drop databases etc. Also possibly with an "use"
// method that you can pass the db name to, and it will return a connection to that
// database.
pub async fn test_conn() -> PgPool {
    let db_name = get_env_var("PG_TEST_DATABASE");
    // TODO: Add logic here to check if db exists, and if not, create it. This is convenient
    // for running tests locally or in CI/CD environments where the database might not exist
    // yet. The above is desirable for tests only and not for production, as in production
    // the database should already exist, and having to check for database existence in
    // this context would impact performance during application usage. For tests it should
    // be fine, however.
    let pool = get_pg_pool(&Some(db_name)).await;
    pool
}
