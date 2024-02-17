use sqlx::PgPool;

const QUERY: &str = "
    CREATE TABLE migrations (
        id SERIAL PRIMARY KEY,
        name VARCHAR(255) NOT NULL,
        applied TIMESTAMP DEFAULT CURRENT_TIMESTAMP
    );
";

pub async fn up(pool: &PgPool) -> Result<(), sqlx::Error> {
    sqlx::query(QUERY).execute(pool).await?;
    Ok(())
}
