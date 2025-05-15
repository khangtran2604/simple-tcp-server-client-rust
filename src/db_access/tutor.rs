use std::fmt::format;

use sqlx::SqliteConnection;

use crate::custom_error::CustomError;
use crate::models::tutor::{NewTutorInput, Tutor};

pub async fn post_tutor_db(
    conn: &mut SqliteConnection,
    data: NewTutorInput,
) -> Result<Tutor, CustomError> {
    let query_str = format!(
        "
    INSERT INTO tutor(name, age) VALUES('{}', {})
    RETURNING id, name, age, created_at
",
        data.name, data.age
    );

    let created_tutor = sqlx::query_as::<_, Tutor>(&query_str)
        .fetch_one(conn)
        .await
        .map_err(|e| CustomError::SqlxError(e.to_string()))?;

    Ok(created_tutor)
}

pub async fn get_tutors_db(conn: &mut SqliteConnection) -> Result<Vec<Tutor>, CustomError> {
    let query_str = "SELECT * from tutor";

    let tutors = sqlx::query_as::<_, Tutor>(query_str)
        .fetch_all(conn)
        .await
        .map_err(|e| CustomError::SqlxError(e.to_string()))?;

    Ok(tutors)
}

pub async fn get_tutor_by_id_db(
    conn: &mut SqliteConnection,
    tutor_id: &str,
) -> Result<Option<Tutor>, CustomError> {
    let query_str = format!(
        "
SELECT * from tutor WHERE id = {}
",
        tutor_id
    );

    let result = sqlx::query_as::<_, Tutor>(&query_str)
        .fetch_optional(conn)
        .await
        .map_err(|e| CustomError::SqlxError(e.to_string()))?;

    Ok(result)
}

#[cfg(test)]
mod tests {
    use dotenv::dotenv;
    use fake::{Fake, faker::name::en::FirstName};

    use crate::db_access::get_db_conn;

    use super::*;

    #[actix_web::test]
    async fn test_post_tutor_db_successfully() {
        dotenv().ok();

        let mut conn = get_db_conn().await;
        let new_tutor_name: String = FirstName().fake();
        let new_tutor_age: i32 = (20..40).fake();

        let new_tutor_input = NewTutorInput {
            name: new_tutor_name,
            age: new_tutor_age,
        };

        let result = post_tutor_db(&mut conn, new_tutor_input.clone()).await;

        println!("{:?}", result);

        assert!(result.is_ok());

        let created_tutor = result.unwrap();

        assert_eq!(created_tutor.name, new_tutor_input.name);
        assert_eq!(created_tutor.age, new_tutor_input.age);
    }

    #[actix_web::test]
    async fn test_get_tutors_db() {
        dotenv().ok();

        let mut conn = get_db_conn().await;
        let result = get_tutors_db(&mut conn).await;

        assert!(result.is_ok());

        let tutors = result.unwrap();

        assert!(!tutors.is_empty())
    }

    #[actix_web::test]
    async fn test_get_tutor_by_id_successfully() {
        dotenv().ok();

        let mut conn = get_db_conn().await;
        let new_tutor_name: String = FirstName().fake();
        let new_tutor_age: i32 = (20..40).fake();

        let new_tutor_input = NewTutorInput {
            name: new_tutor_name,
            age: new_tutor_age,
        };

        let created_tutor = post_tutor_db(&mut conn, new_tutor_input).await.unwrap();

        let find_result = get_tutor_by_id_db(&mut conn, created_tutor.id.to_string().trim()).await;

        assert!(find_result.is_ok());

        let option_tutor = find_result.unwrap();
        assert!(option_tutor.is_some());

        let tutor = option_tutor.unwrap();

        assert_eq!(created_tutor, tutor);
    }

    #[actix_web::test]
    async fn test_get_non_exist_tutor_by_id() {
        dotenv().ok();

        let mut conn = get_db_conn().await;
        let non_exist_tutor_id = "99999999991";

        let result = get_tutor_by_id_db(&mut conn, non_exist_tutor_id).await;
        println!("{:?}", result);

        assert!(result.is_ok());

        let option_tutor = result.unwrap();

        assert!(option_tutor.is_none());
    }
}
