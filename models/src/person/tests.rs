use chrono::NaiveDate;
use dotenv::dotenv;
use fake::faker::chrono::raw::Date;
use fake::faker::name::raw::{FirstName, LastName};
use fake::locales::EN;
use fake::Fake;
use fake::Faker;
use models::{
    person::create::Create, person::delete::Delete, person::read::get_person_by_id_optional,
    person::update::Update, NewPerson, Person,
};
use shared::get_pg_pool::{conn, test_conn};
use sqlx::query;
use sqlx::Row;

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
async fn test_create_and_read_person() {
    dotenv().ok();

    let pool = conn().await;

    for _ in 0..10 {
        let new_person = generate_test_person();

        let result = new_person
            .create(&pool)
            .await
            .expect("Failed to create new person");

        let inserted_person = get_person_by_id_optional(&result.id, &pool).await.unwrap();

        match inserted_person {
            Some(inserted_person) => {
                assert_eq!(inserted_person.first_name, new_person.first_name);
                assert_eq!(inserted_person.last_name, new_person.last_name);
                assert_eq!(inserted_person.date_of_birth, new_person.date_of_birth);
            }
            None => panic!("Failed to fetch inserted person from the database"),
        }
    }
}

#[tokio::test]
async fn test_update_person() {
    dotenv().ok();

    let pool = conn().await;

    for _ in 0..10 {
        let new_person: NewPerson = generate_test_person();
        let person = new_person.create(&pool).await.unwrap();

        let mut person_from_db: Option<Person> =
            get_person_by_id_optional(&person.id, &pool).await.unwrap();

        let updated_first_name: String = FirstName(EN).fake();
        let updated_last_name: String = LastName(EN).fake();
        let updated_date_of_birth: Option<chrono::NaiveDate> = Some(Date(EN).fake());

        match &mut person_from_db {
            Some(person_from_db) => {
                person_from_db.first_name = updated_first_name.clone();
                person_from_db.last_name = updated_last_name.clone();
                person_from_db.date_of_birth = updated_date_of_birth.clone();

                person_from_db.update(&pool).await.unwrap();

                let updated_person_from_db =
                    get_person_by_id_optional(&person.id, &pool).await.unwrap();

                match updated_person_from_db {
                    Some(updated_person_from_db) => {
                        assert_eq!(updated_person_from_db.first_name, updated_first_name);
                        assert_eq!(updated_person_from_db.last_name, updated_last_name);
                        assert_eq!(updated_person_from_db.date_of_birth, updated_date_of_birth);
                    }
                    None => panic!("Failed to fetch updated person from the database"),
                }
            }
            None => panic!("Failed to fetch person from the database"),
        }
    }
}

#[tokio::test]
async fn test_delete_person() {
    dotenv().ok();

    let pool = conn().await;

    for _ in 0..10 {
        let new_person: NewPerson = generate_test_person();
        let person = new_person.create(&pool).await.unwrap();

        let mut person_from_db: Option<Person> =
            get_person_by_id_optional(&person.id, &pool).await.unwrap();

        match &mut person_from_db {
            Some(person_from_db) => {
                person_from_db.delete(&pool).await.unwrap();

                let check_if_inexistent = query!("SELECT * FROM persons WHERE id = $1", person.id)
                    .fetch_optional(&pool)
                    .await
                    .expect("Failed to fetch updated person from the database");

                assert!(check_if_inexistent.is_none());
            }
            None => panic!("Failed to fetch person from the database"),
        }
    }
}
