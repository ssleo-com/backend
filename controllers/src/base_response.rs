use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct ErrorResponse {
    message: String,
    error_code: i32,
}

#[derive(Debug, Serialize, Deserialize)]
struct ManyResponse<T> {
    #[serde(flatten)]
    data: Vec<T>,
}
