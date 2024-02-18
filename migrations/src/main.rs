mod m0002_create_persons_table;
use shared::{does_table_exist, get_env_var, get_pg_pool::get_pg_pool};

#[tokio::main]
async fn main() {
    let pool = get_pg_pool().await;

    let exists = does_table_exist("migrations", &pool).await;

    match exists {
        true => println!("table exists"),
        false => println!("table does not exist"),
    }

    // m0002_create_persons_table::down::down(&pool).await.unwrap();
    // m0002_create_persons_table::up::up(&pool).await.unwrap();
}
