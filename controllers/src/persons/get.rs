use axum::{extract::Path, Json};

use models::person::read::get_person_by_id_optional;
use models::Person;

use shared::get_pg_pool::conn;

// The below is actually the get_single handler. Please update the below to
// actually return a list of persons.
pub async fn get_handler(Path(id): Path<i32>) -> Json<Person> {
    let pool = conn().await;

    println!("ID: {:?}", id);

    let person = get_person_by_id_optional(&id, &pool).await.unwrap();

    let result = match person {
        Some(person) => Json(person),
        None => {
            std::process::exit(1);
        }
    };

    result
}
