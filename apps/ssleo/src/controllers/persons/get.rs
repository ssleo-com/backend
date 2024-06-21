use axum::{extract::Path, Json};

use crate::models::person::read::get_multiple_persons;
use crate::models::Person;

use crate::shared::get_env_var::get_env_var;
use crate::shared::get_pg_pool::get_pg_pool;

use crate::controllers::base_response::ManyResponse;

pub async fn get_multiple_handler() -> Json<ManyResponse<Person>> {
    let db_name = get_env_var("PG_DATABASE");
    let pool = get_pg_pool(&Some(db_name)).await;

    let person = get_multiple_persons(&pool).await.unwrap();

    let result = ManyResponse { data: person };

    Json(result)
}
