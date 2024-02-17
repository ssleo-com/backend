mod m0002_create_persons_table;

const URL: &str = "postgres://admin:admin@database:5432/admin";

#[tokio::main]
async fn main() {
    let pool = sqlx::postgres::PgPool::connect(URL).await.unwrap();

    m0002_create_persons_table::down::down(&pool).await.unwrap();
    m0002_create_persons_table::up::up(&pool).await.unwrap();
}
