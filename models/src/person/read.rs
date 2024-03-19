use sqlx::PgPool;

use crate::Person;

use super::person::SimplePerson;

pub async fn get_person_by_id_optional(
    id: &i32,
    pool: &PgPool,
) -> Result<Option<Person>, sqlx::Error> {
    let row = sqlx::query_as!(
        Person,
        r#"
            SELECT 
                id,
                first_name, 
                last_name, 
                date_of_birth, 
                created_at, 
                updated_at
            FROM persons 
            WHERE id = $1
            "#,
        id
    )
    .fetch_optional(pool)
    .await?;

    Ok(row)
}

pub async fn get_simple_person_by_id(id: &i32, pool: &PgPool) -> Result<SimplePerson, sqlx::Error> {
    let row = sqlx::query_as!(
        SimplePerson,
        r#"
            SELECT 
                id,
                first_name, 
                last_name
            FROM persons 
            WHERE id = $1
            "#,
        id
    )
    .fetch_one(pool)
    .await?;

    Ok(row)
}

pub async fn get_multiple_persons(pool: &PgPool) -> Result<Vec<Person>, sqlx::Error> {
    let rows = sqlx::query_as!(
        Person,
        r#"
            SELECT 
                id,
                first_name, 
                last_name, 
                date_of_birth, 
                created_at, 
                updated_at
            FROM persons
        "#
    )
    .fetch_all(pool)
    .await?;

    Ok(rows)
}

// Here we will need to continue with more implementations,
// such as searching by name (which would include both first
// and last), searching by date of birth, and so on. Perhaps
// something that can be achieved by building a general
// filtering/searching functionality. We'll see.
