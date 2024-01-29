use sqlx::PgPool;

const QUERY: &str = "DROP TABLE persons;";

pub async fn down(pool: &PgPool) -> Result<(), sqlx::Error> {
    sqlx::query(QUERY).execute(pool).await?;
    Ok(())
}
