use axum::{extract::Path, Json};

use models::person::read::get_multiple_persons;
use models::Person;

use shared::get_pg_pool::conn;

use crate::base_response::ManyResponse;

pub async fn get_multiple_handler() -> Json<ManyResponse<Person>> {
    let pool = conn().await;

    let person = get_multiple_persons(&pool).await.unwrap();

    let result = ManyResponse { data: person };

    Json(result)
}
