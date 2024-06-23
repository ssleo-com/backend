use chrono::{DateTime, NaiveDate, Utc};
use comfy_table::{ContentArrangement, Table};
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use std::{error::Error, time::Instant};
use structs::Person;

pub fn query() -> Result<(), Box<dyn Error>> {
    // Create a reqwest client
    let client = Client::new();

    // Make a GET request to localhost:9090/persons
    let url = "http://localhost:9090/persons";

    let start_time = Instant::now();

    let response = client.get(url).send();

    match response {
        Ok(res) => {
            let elapsed_time = start_time.elapsed();
            println!("Req duration: {:?}", elapsed_time);

            if res.status().is_success() {
                // Deserialize JSON response to a structure matching your data
                let json_data: serde_json::Value = res.json()?;
                let persons_json = &json_data["data"];

                // Deserialize each person JSON object into a Vec<Person>
                let mut persons: Vec<Person> = Vec::new();
                for person_json in persons_json.as_array().unwrap() {
                    let person: Person = serde_json::from_value(person_json.clone()).unwrap();
                    persons.push(person);
                }

                // Create a new table
                let mut table = Table::new();

                // Define table headers
                table.set_header(vec![
                    "ID".to_string(),
                    "First Name".to_string(),
                    "Last Name".to_string(),
                    "Date of Birth".to_string(),
                    "Created At".to_string(),
                    "Updated At".to_string(),
                ]);

                // Add rows to the table
                for person in &persons {
                    table.add_row(vec![
                        person.id.to_string(),
                        person.first_name.clone(),
                        person.last_name.clone(),
                        person
                            .date_of_birth
                            .map_or_else(|| "".to_string(), |dob| dob.to_string()),
                        person
                            .created_at
                            .map_or_else(|| "".to_string(), |dt| dt.to_string()),
                        person
                            .updated_at
                            .map_or_else(|| "".to_string(), |dt| dt.to_string()),
                    ]);
                }

                // Print the table
                table.set_content_arrangement(ContentArrangement::Dynamic);
                println!("{}", table);
            } else {
                eprintln!("Error: HTTP {}", res.status());
                let body = res.text()?;
                eprintln!("Response body:\n{}", body);
            }
        }
        Err(err) => {
            eprintln!("Error executing request: {}", err);
        }
    }

    Ok(())
}
