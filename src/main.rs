use axum::{response::Html, routing::get, Router};
use sqlx::Row;

async fn handler() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}

#[tokio::main]
async fn main() {
    let url = "postgres://admin:admin@database:5432/admin";

    let pool = sqlx::postgres::PgPool::connect(url).await.unwrap();

    let res = sqlx::query("SELECT 1 + 1 AS sum")
        .fetch_one(&pool)
        .await
        .unwrap();

    sqlx::query(r#"DROP TABLE ticket;"#)
        .execute(&pool)
        .await
        .unwrap();

    let sum: i32 = res.get("sum");
    println!("sum: {}", sum);
    let app = Router::new().route("/", get(handler));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
