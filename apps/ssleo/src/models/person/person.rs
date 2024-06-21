use chrono::{NaiveDate, Utc};

// Add the missing generic argument for DateTime
use chrono::DateTime;
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
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

// The NewPerson struct mirrors the JSON structure of a person,
// coming from the client. Id and timestamps are not included,
// as they are generatet by the database.
#[derive(Serialize)]
pub struct NewPerson {
    pub first_name: String,
    pub last_name: String,
    pub date_of_birth: Option<NaiveDate>,
}

// The PatchPerson struct mirrors the JSON structure of a person,
// coming from the client, during a patch request. It includes
// optional fields, as the client may not want to update all
// fields, thus the reason to use the patch method instead of put.
#[derive(Serialize)]
pub struct PatchPerson {
    pub id: i32,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub date_of_birth: Option<NaiveDate>,
}

// The SimplePerson struct is a simplified version of the Person
// struct, and it is used for endpoints that do not require a full
// version of the record, such as to fill the options of a dropdown.
#[derive(Serialize)]
pub struct SimplePerson {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
}
