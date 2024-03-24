use axum::Json;
use chrono::{NaiveDate, NaiveDateTime};

use crate::models::person::update::Update;
use crate::models::Person;
use crate::shared::get_env_var::get_env_var;
use crate::shared::get_pg_pool::get_pg_pool;

pub async fn patch_handler() -> Json<Person> {
    let db_name = get_env_var("PG_DATABASE");

    let pool = get_pg_pool(&Some(db_name)).await;

    let from_ymd_opt = NaiveDate::from_ymd_opt;
    let from_ymd_opt2 = NaiveDateTime::from_timestamp_opt;

    let new_user = Person {
        id: 1,
        first_name: String::from("Jane"),
        last_name: String::from("Doe"),
        date_of_birth: from_ymd_opt(2015, 3, 14),
        created_at: from_ymd_opt2(59, 1_500_000_000),
        updated_at: from_ymd_opt2(59, 1_500_000_000),
    };

    new_user.update(&pool).await.unwrap();

    Json(new_user)
}
