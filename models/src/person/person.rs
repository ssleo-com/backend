use chrono::{NaiveDate, NaiveDateTime};
use serde::Serialize;

// The Person struct represents a data structure that
// holds information about a person, in its database
// structure format. It tries to mirror the database
// column definitions, and this is how a person record
// should look like when selected via a select all query.
#[derive(Serialize)]
pub struct Person {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub date_of_birth: Option<NaiveDate>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

// The NewPerson struct represents a data structure that
// holds information about a person. It mirrors the
// JSON structure of a person, coming from the client.
// Id and timestamps are not included, as they are generated
// by the database.
#[derive(Serialize)]
pub struct NewPerson {
    pub first_name: String,
    pub last_name: String,
    pub date_of_birth: Option<NaiveDate>,
}

#[derive(Serialize)]
pub struct PatchPerson {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub date_of_birth: Option<NaiveDate>,
}
