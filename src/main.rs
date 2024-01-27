use axum::{response::Html, routing::get, Router};
use sqlx::Connection;
use sqlx::Row;

#[tokio::main]
async fn main() {
    let url = "postgres://admin:admin@database:5432/admin";

    let mut conn = sqlx::postgres::PgConnection::connect(url).await.unwrap();

    let res = sqlx::query("SELECT 1 + 1 AS sum")
        .fetch_one(&mut conn)
        .await
        .unwrap();

    let sum: i32 = res.get("sum");
    println!("sum: {}", sum);
    let app = Router::new().route("/", get(handler));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn handler() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}
