use axum::{routing::get, Json, Router};
use dotenv::dotenv;

use models::person::{NewPerson, Person};
use shared::get_pg_pool::get_pg_pool;

async fn handler() -> Json<Person> {
    let user = Person {
        id: 1,
        first_name: String::from("John"),
        last_name: String::from("Doe"),
        date_of_birth: Some(String::from("1990-01-01")),
        created_at: String::from("2021-01-01"),
        updated_at: String::from("2021-01-01"),
    };

    Json(user)
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let pool = get_pg_pool().await;

    let app = Router::new().route("/", get(handler));

    let new_user = NewPerson {
        first_name: String::from("Jane"),
        last_name: String::from("Doe"),
        date_of_birth: Some(String::from("1990-01-01")),
    };

    new_user.save(&pool).await.unwrap();

    let listener = tokio::net::TcpListener::bind("0.0.0.0:9090").await.unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
