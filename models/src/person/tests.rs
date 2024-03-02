use chrono::NaiveDate;
use dotenv::dotenv;
use fake::faker::chrono::raw::Date;
use fake::faker::name::raw::{FirstName, LastName};
use fake::locales::EN;
use fake::Fake;
use fake::Faker;
use models::{person::create::Create, person::update::Update, NewPerson};
use shared::get_pg_pool::test_conn;
use sqlx::query;

fn generate_test_person() -> NewPerson {
    let from_ymd_opt = NaiveDate::from_ymd_opt;
    let first_name: String = FirstName(EN).fake();
    let last_name: String = LastName(EN).fake();
    let date_of_birth: Option<chrono::NaiveDate> = Some(Date(EN).fake());

    let new_user = NewPerson {
        first_name,
        last_name,
        date_of_birth,
    };

    new_user
}

#[tokio::test]
async fn test_create_person() {
    dotenv().ok();

    let pool = test_conn().await;

    for _ in 0..10 {
        let new_person = generate_test_person();

        let result = new_person
            .create(&pool)
            .await
            .expect("Failed to create new person");

        let inserted_person = query!(
            "SELECT first_name, last_name, date_of_birth FROM persons WHERE id = $1",
            result.id
        )
        .fetch_one(&pool)
        .await
        .expect("Failed to fetch inserted person from the database");

        assert_eq!(inserted_person.first_name, new_person.first_name);
        assert_eq!(inserted_person.last_name, new_person.last_name);
        assert_eq!(inserted_person.date_of_birth, new_person.date_of_birth);
    }
}

#[tokio::test]
async fn test_update_person() {
    dotenv().ok();

    let pool = test_conn().await;

    for _ in 0..10 {
        // Create a new person
        let mut new_person = generate_test_person();
        let result = new_person.create(&pool).await.unwrap();

        // Update the person's information
        let new_person.first_name = FirstName(EN).fake();
        let updated_last_name = LastName(EN).fake();
        let updated_date_of_birth: Option<chrono::NaiveDate> = Some(Date(EN).fake());

        // Perform the update operation
        new_person.update(&updated_person, &pool).await.unwrap();

        // Fetch the updated person from the database
        let updated_person_from_db = query!("SELECT * FROM persons WHERE id = $1", result.id)
            .fetch_one(&pool)
            .await
            .expect("Failed to fetch updated person from the database");

        // Assert that the person's information has been updated correctly
        assert_eq!(updated_person_from_db.first_name, updated_first_name);
        assert_eq!(updated_person_from_db.last_name, updated_last_name);
        assert_eq!(updated_person_from_db.date_of_birth, updated_date_of_birth);
    }
}
