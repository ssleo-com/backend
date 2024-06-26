use axum::Json;
use chrono::{DateTime, NaiveDate, TimeZone, Utc};

use crate::models::person::update::Update;
use crate::shared::get_env_var::get_env_var;
use crate::shared::get_pg_pool::get_pg_pool;
use structs::Person;

pub async fn patch_handler() -> Json<Person> {
    let db_name = get_env_var("PG_DATABASE");

    let pool = get_pg_pool(&Some(db_name)).await;

    let from_ymd_opt = NaiveDate::from_ymd_opt;

    let from_ymd_opt2 = TimeZone::from_utc_datetime;

    let naive_date_time = NaiveDate::from_ymd_opt(2021, 1, 1)
        .expect("")
        .and_hms_opt(0, 0, 0)
        .expect("");

    let new_user = Person {
        id: 1,
        first_name: String::from("Jane"),
        last_name: String::from("Doe"),
        date_of_birth: from_ymd_opt(2015, 3, 14),
        created_at: Some(from_ymd_opt2(&Utc, &naive_date_time)),
        updated_at: Some(from_ymd_opt2(&Utc, &naive_date_time)),
    };

    new_user.update(&pool).await.unwrap();

    Json(new_user)
}
