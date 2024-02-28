use axum::{routing::get, Json, Router};
use chrono::NaiveDate;
use dotenv::dotenv;
use models::person::NewPerson;
use models::person::Save;
use shared::get_pg_pool::get_pg_pool;

async fn handler() -> Json<NewPerson> {
    let pool = get_pg_pool().await;

    let from_ymd_opt = NaiveDate::from_ymd_opt;

    let new_user = NewPerson {
        first_name: String::from("Jane"),
        last_name: String::from("Doe"),
        date_of_birth: from_ymd_opt(2015, 3, 14),
    };

    new_user.save(&pool).await.unwrap();

    Json(new_user)
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let app = Router::new().route("/", get(handler));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:9090").await.unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
