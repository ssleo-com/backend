use sqlx::PgPool;

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
