use axum::{extract::Path, Json};

use crate::models::person::read::get_person_by_id_optional;
use structs::Person;

use crate::shared::get_env_var::get_env_var;
use crate::shared::get_pg_pool::get_pg_pool;
use sqlx::PgPool;

pub async fn get_single_handler(Path(id): Path<i32>, pool: &PgPool) -> Json<Person> {
    let db_name = get_env_var("PG_DATABASE");
    let pool = get_pg_pool(&Some(db_name)).await;

    println!("ID: {:?}", id);

    let person = get_person_by_id_optional(&id, &pool).await.unwrap();

    let result = match person {
        Some(person) => Json(person),
        None => {
            std::process::exit(1);
        }
    };

    result
}
