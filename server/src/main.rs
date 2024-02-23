use axum::{response::Html, routing::get, Router};
use dotenv::dotenv;
use shared::get_pg_pool::get_pg_pool;
use sqlx::{PgPool, Row};
use std::env;

async fn handler() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
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
