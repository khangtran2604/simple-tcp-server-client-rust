use sqlx::SqliteConnection;

use crate::custom_error::CustomError;
use crate::models::tutor::{NewTutorInput, Tutor, UpdateTurtorInput};

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

pub async fn patch_tutor_by_id_db(
    conn: &mut SqliteConnection,
    tutor_id: &str,
    update_data: UpdateTurtorInput,
) -> Result<(), CustomError> {
    let tutor = get_tutor_by_id_db(conn, tutor_id).await?;

    if tutor.is_none() {
        return Err(CustomError::SqlxError("Tutor was not found !".to_string()));
    };
    let tutor = tutor.unwrap();

    let update_name = if update_data.name.is_some() {
        update_data.name.unwrap()
    } else {
        tutor.name
    };

    let update_age = if update_data.age.is_some() {
        update_data.age.unwrap()
    } else {
        tutor.age
    };

    let query_str = format!(
        "
    UPDATE tutor SET name = '{}', age = {}
    WHERE id = {}
",
        update_name, update_age, tutor_id
    );

    sqlx::query(&query_str)
        .execute(conn)
        .await
        .map_err(|e| CustomError::SqlxError(e.to_string()))?;

    Ok(())
}

pub async fn del_tutor_by_id_db(
    conn: &mut SqliteConnection,
    tutor_id: &str,
) -> Result<(), CustomError> {
    let find_tutor = get_tutor_by_id_db(conn, tutor_id).await?;

    if find_tutor.is_none() {
        return Err(CustomError::InvalidInputData(
            "Tutor was not found!".to_string(),
        ));
    }

    let query_str = format!(
        "
    DELETE FROM tutor WHERE id = {}
",
        tutor_id
    );

    sqlx::query(&query_str)
        .execute(conn)
        .await
        .map_err(|e| CustomError::SqlxError(e.to_string()))?;

    Ok(())
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

    #[actix_web::test]
    async fn test_patch_non_exist_tutor_db() {
        dotenv().ok();

        let mut conn = get_db_conn().await;
        let non_exists_tutor = "9999999999991";
        let update_data_input = UpdateTurtorInput {
            name: FirstName().fake(),
            age: (20..40).fake(),
        };

        let result = patch_tutor_by_id_db(&mut conn, non_exists_tutor, update_data_input).await;

        assert!(result.is_err());
    }

    #[actix_web::test]
    async fn test_patch_exist_tutor_db() {
        dotenv().ok();

        let mut conn = get_db_conn().await;
        let new_tutor_input = NewTutorInput {
            name: FirstName().fake(),
            age: (20..40).fake(),
        };

        let new_tutor = post_tutor_db(&mut conn, new_tutor_input.clone())
            .await
            .unwrap();

        let update_tutor_input = UpdateTurtorInput {
            name: Some(FirstName().fake()),
            age: None,
        };

        let result = patch_tutor_by_id_db(
            &mut conn,
            new_tutor.id.to_string().trim(),
            update_tutor_input.clone(),
        )
        .await;

        println!("{:?}", result);
        // Update tutor is ok
        assert!(result.is_ok());

        let updated_tutor = get_tutor_by_id_db(&mut conn, new_tutor.id.to_string().trim())
            .await
            .unwrap()
            .unwrap();

        // Data must update based on input data
        assert_eq!(updated_tutor.name, update_tutor_input.name.unwrap());
        assert_eq!(updated_tutor.age, new_tutor_input.age);
    }

    #[actix_web::test]
    async fn test_del_non_exist_tutor_by_id_db() {
        dotenv().ok();

        let mut conn = get_db_conn().await;
        let non_exist_tutor_id = "99999999991";

        let result = del_tutor_by_id_db(&mut conn, non_exist_tutor_id).await;

        assert!(result.is_err());
    }

    #[actix_web::test]
    async fn test_del_exist_tutor_by_id_db() {
        dotenv().ok();

        let mut conn = get_db_conn().await;
        let new_tutor_input = NewTutorInput {
            name: FirstName().fake(),
            age: (20..40).fake(),
        };

        let new_tutor = post_tutor_db(&mut conn, new_tutor_input.clone())
            .await
            .unwrap();

        let result = del_tutor_by_id_db(&mut conn, new_tutor.id.to_string().trim()).await;

        assert!(result.is_ok());
    }
}
