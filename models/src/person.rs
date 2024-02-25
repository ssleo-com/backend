use serde::Serialize;

#[derive(Serialize)]
pub struct Person {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub date_of_birth: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}
