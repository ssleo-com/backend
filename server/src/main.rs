use axum::{extract::Path, routing::get, Json, Router};
use dotenv::dotenv;
use serde::Serialize;
use shared::get_pg_pool::get_pg_pool;
use sqlx::{PgPool, Row};
use std::env;

#[derive(Serialize)]
struct Person {
    id: i32,
    name: String,
    age: i32,
    city: String,
}

async fn handler() -> Json<Person> {
    let user = Person {
        id: 1,
        name: String::from("John"),
        age: 30,
        city: String::from("New York"),
    };

    Json(user)
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let pool = get_pg_pool().await;

    let app = Router::new().route("/", get(handler));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:9090").await.unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
