use std::time::{SystemTime, UNIX_EPOCH};

use axum::{extract::Path, Json};
use axum::{routing::get, Router};
mod controllers;
use controllers::{
    delete_handler, get_multiple_handler, get_single_handler, patch_handler, post_handler,
};
use dotenv::dotenv;
mod models;
mod shared;
use serde::{Deserialize, Serialize};
use shared::{get_env_var::get_env_var, get_pg_pool::get_pg_pool};
use sqlx::PgPool;

use jsonwebtoken::{encode, EncodingKey, Header};

/// Our claims struct, it needs to derive `Serialize` and/or `Deserialize`
#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    company: String,
    exp: usize,
    iat: i64,
}

fn issue_jwt(user_id: usize) -> Result<String, Box<dyn std::error::Error>> {
    let now = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
    // Define your payload (claims) - for example, the user ID
    let claims = Claims {
        sub: user_id.to_string(), // Subject
        company: user_id.to_string(),
        exp: 10000000000, // Expiration time
        iat: now as i64,
    };

    // Define your secret key
    let secret = "your_secret_key";

    // Encode the JWT using the HS256 algorithm and your secret key
    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_ref()),
    )?;

    Ok(token)
}

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

    println!("{}", issue_jwt(1).unwrap());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:9090").await.unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
