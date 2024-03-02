use axum::{routing::get, Router};
use controllers::{delete_handler, get_handler, get_single_handler, patch_handler, post_handler};
use dotenv::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let app = Router::new()
        // TODO: The below needs to be updated to take into account bulk create,
        // bulk update, and bulk delete.
        .route("/persons", get(get_handler).post(post_handler))
        .route(
            "/persons/:id",
            get(get_single_handler)
                .patch(patch_handler)
                .delete(delete_handler),
        );

    let listener = tokio::net::TcpListener::bind("0.0.0.0:9090").await.unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
