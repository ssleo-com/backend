use axum::{extract::Path, Json};
use axum::{routing::get, Router};
mod controllers;
use controllers::{
    delete_handler, get_multiple_handler, get_single_handler, patch_handler, post_handler,
};
use dotenv::dotenv;
mod models;
mod shared;
use shared::{get_env_var::get_env_var, get_pg_pool::get_pg_pool};
use sqlx::PgPool;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let db_name = get_env_var("PG_DATABASE");
    let pool = get_pg_pool(&Some(db_name)).await;

    let app = Router::new()
        // TODO: The below needs to be updated to take into account bulk create,
        // bulk update, and bulk delete.
        .route("/persons", get(get_multiple_handler).post(post_handler))
        .route(
            "/persons/:id",
            get(|Path(id): Path<i32>| async move { get_single_handler(Path(id), &pool).await })
                .patch(patch_handler)
                .delete(delete_handler),
        );

    let listener = tokio::net::TcpListener::bind("0.0.0.0:9090").await.unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
