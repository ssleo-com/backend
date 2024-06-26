use axum::Json;
use chrono::NaiveDate;

use crate::models::person::create::Create;
use crate::shared::get_env_var::get_env_var;
use crate::shared::get_pg_pool::get_pg_pool;
use structs::NewPerson;

pub async fn post_handler() -> Json<NewPerson> {
    let db_name = get_env_var("PG_DATABASE");
    let pool = get_pg_pool(&Some(db_name)).await;

    let from_ymd_opt = NaiveDate::from_ymd_opt;

    let new_user = NewPerson {
        first_name: String::from("Jane"),
        last_name: String::from("Doe"),
        date_of_birth: from_ymd_opt(2015, 3, 14),
    };

    new_user.create(&pool).await.unwrap();

    Json(new_user)
}
