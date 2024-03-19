use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorResponse {
    message: String,
    error_code: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ManyResponse<T> {
    pub(crate) data: Vec<T>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SingleResponse<T> {
    // #[serde(flatten)] leaving this here until proven not needed
    pub(crate) data: T,
}
