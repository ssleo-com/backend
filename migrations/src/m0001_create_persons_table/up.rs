use sqlx::PgPool;
mod create_table;

const QUERY: &str = "
    CREATE TABLE persons (
        id SERIAL PRIMARY KEY,
        first_name VARCHAR(255) NOT NULL,
        last_name VARCHAR(255) NOT NULL,
        date_of_birth DATE,
        created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
        updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
    );
";

pub async fn up(pool: &PgPool) -> Result<(), sqlx::Error> {
    sqlx::query(QUERY).execute(pool).await?;
    Ok(())
}
