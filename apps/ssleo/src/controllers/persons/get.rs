use axum::extract::Query;
use axum::{extract::Path, Json};
use serde::{Deserialize, Serialize};

use crate::models::person::read::get_multiple_persons;
use structs::{Person, SearchPerson};

use crate::shared::get_env_var::get_env_var;
use crate::shared::get_pg_pool::get_pg_pool;

use crate::controllers::base_response::ManyResponse;

#[derive(Serialize, Deserialize, Debug)]
pub struct Filter {
    pub filters: String,
}

pub async fn get_multiple_handler(Query(query): Query<Filter>) -> Json<ManyResponse<Person>> {
    let decoded = serde_urlencoded::from_str::<Vec<(String, String)>>(&query.filters.as_str());

    println!("decoded: {:?}", decoded);
    println!("query_params: {:?}", query);

    let p: SearchPerson = serde_json::from_str(query.filters.as_str()).unwrap();
    println!("{p:?}");
    let db_name = get_env_var("PG_DATABASE");
    let pool = get_pg_pool(&Some(db_name)).await;

    let person = get_multiple_persons(&pool).await.unwrap();

    let result = ManyResponse { data: person };

    Json(result)
}

// async fn get_multiple_handler(Query(query_params): Query<MyQueryParams>) -> Json<Vec<Person>> {
//     // Access query parameters
//     let limit = query_params.limit.unwrap_or(10); // Default limit or use a sensible default

//     // Perform logic to fetch multiple persons based on query parameters
//     let persons: Vec<Person> = fetch_persons_from_database(limit).await;

//     // Return JSON response with persons
//     Json(persons)
// }
