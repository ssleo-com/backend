use sqlx::PgPool;

const QUERY: &str = "DROP TABLE migrations;";

pub async fn down(pool: &PgPool) -> Result<(), sqlx::Error> {
    sqlx::query(QUERY).execute(pool).await?;
    Ok(())
}
