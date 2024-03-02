use axum::Json;
use chrono::NaiveDate;
use models::person::create::Create;
use models::NewPerson;
use shared::get_pg_pool::get_pg_pool;

pub async fn post_handler() -> Json<NewPerson> {
    let pool = get_pg_pool().await;

    let from_ymd_opt = NaiveDate::from_ymd_opt;

    let new_user = NewPerson {
        first_name: String::from("Jane"),
        last_name: String::from("Doe"),
        date_of_birth: from_ymd_opt(2015, 3, 14),
    };

    new_user.create(&pool).await.unwrap();

    Json(new_user)
}
