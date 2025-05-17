use sqlx::SqliteConnection;

use crate::{
    custom_error::CustomError,
    models::course::{Course, NewCourseInput, UpdateCourseInput},
};

pub async fn get_course_by_id_db(
    conn: &mut SqliteConnection,
    course_id: &str,
) -> Result<Option<Course>, CustomError> {
    let query_str = format!(
        "
    SELECT * FROM course WHERE id = {}
    ",
        course_id
    );

    let result = sqlx::query_as::<_, Course>(&query_str)
        .fetch_optional(conn)
        .await
        .map_err(|e| CustomError::SqlxError(e.to_string()))?;

    Ok(result)
}

pub async fn get_courses_db(conn: &mut SqliteConnection) -> Result<Vec<Course>, CustomError> {
    let query_str = "SELECT * FROM course";

    let courses = sqlx::query_as::<_, Course>(query_str)
        .fetch_all(conn)
        .await
        .map_err(|e| CustomError::SqlxError(e.to_string()))?;

    Ok(courses)
}

pub async fn post_course_db(
    conn: &mut SqliteConnection,
    data: NewCourseInput,
) -> Result<Course, CustomError> {
    let query_str = format!(
        "
    INSERT INTO course(tutor_id, name, description) VALUES('{}', '{}', '{}')
    RETURNING id, name, tutor_id, description, posted_time
    ",
        data.tutor_id,
        data.name,
        data.description.unwrap_or_default()
    );

    let created_course = sqlx::query_as::<_, Course>(&query_str)
        .fetch_one(conn)
        .await
        .map_err(|e| CustomError::SqlxError(e.to_string()))?;

    Ok(created_course)
}

pub async fn patch_course_by_id_db(
    conn: &mut SqliteConnection,
    course_id: &str,
    update_data: UpdateCourseInput,
) -> Result<(), CustomError> {
    let course = get_course_by_id_db(conn, course_id).await?;

    if course.is_none() {
        return Err(CustomError::SqlxError("Course was not found !".to_string()));
    };
    let course = course.unwrap();

    let query_str = format!(
        "
    UPDATE course SET tutor_id = {}, name = '{}', description = '{}' WHERE id = {}
    ",
        update_data.tutor_id.unwrap_or(course.tutor_id),
        update_data.name.unwrap_or(course.name),
        update_data.description.unwrap_or(course.description),
        course_id
    );

    sqlx::query(&query_str)
        .execute(conn)
        .await
        .map_err(|e| CustomError::SqlxError(e.to_string()))?;

    Ok(())
}

pub async fn delete_course_by_id_db(
    conn: &mut SqliteConnection,
    course_id: &str,
) -> Result<(), CustomError> {
    let course = get_course_by_id_db(conn, course_id).await?;

    if course.is_none() {
        return Err(CustomError::SqlxError("Course was not found !".to_string()));
    };

    let query_str = format!(
        "
    DELETE FROM course WHERE id = {}
    ",
        course_id
    );

    sqlx::query(&query_str)
        .execute(conn)
        .await
        .map_err(|e| CustomError::SqlxError(e.to_string()))?;

    Ok(())
}

pub async fn get_courses_by_tutor_id_db(
    conn: &mut SqliteConnection,
    tutor_id: &str,
) -> Result<Vec<Course>, CustomError> {
    let query_str = format!(
        "
    SELECT * FROM course WHERE tutor_id = {}
    ",
        tutor_id
    );

    let courses = sqlx::query_as::<_, Course>(&query_str)
        .fetch_all(conn)
        .await
        .map_err(|e| CustomError::SqlxError(e.to_string()))?;

    Ok(courses)
}

#[cfg(test)]
mod tests {
    use crate::{
        db_access::{get_db_conn, tutor::post_tutor_db},
        models::tutor::{NewTutorInput, Tutor},
    };

    use super::*;
    use dotenv::dotenv;
    use fake::{Fake, faker::name::en::Name};

    async fn _create_new_tutor(conn: &mut SqliteConnection) -> Tutor {
        let new_tutor_name: String = Name().fake();
        let new_tutor_age: i32 = (20..40).fake();
        let new_tutor_input = NewTutorInput {
            name: new_tutor_name,
            age: new_tutor_age,
        };

        let tutor = post_tutor_db(conn, new_tutor_input.clone()).await.unwrap();

        tutor
    }

    #[actix_web::test]
    async fn test_post_course_db_with_non_exist_tutor_id() {
        dotenv().ok();
        let mut conn = get_db_conn().await;

        let non_existing_tutor_id = 99999999;
        let new_course_name: String = Name().fake();
        let new_course = NewCourseInput {
            tutor_id: non_existing_tutor_id,
            name: new_course_name,
            description: Some("This is a test course".to_string()),
        };

        let created_course = post_course_db(&mut conn, new_course).await;

        println!("Created course: {:?}", created_course);

        assert!(created_course.is_err());
    }

    #[actix_web::test]
    async fn test_post_course_db_successfully() {
        dotenv().ok();
        let mut conn = get_db_conn().await;

        let tutor = _create_new_tutor(&mut conn).await;

        let new_course_name: String = Name().fake();
        let new_course = NewCourseInput {
            tutor_id: tutor.id,
            name: new_course_name,
            description: Some("This is a test course".to_string()),
        };

        let created_course = post_course_db(&mut conn, new_course.clone()).await;

        assert!(created_course.is_ok());
        let created_course = created_course.unwrap();

        assert_eq!(created_course.name, new_course.name);
        assert_eq!(created_course.tutor_id, new_course.tutor_id);
        assert_eq!(created_course.description, new_course.description.unwrap());
    }

    #[actix_web::test]
    async fn test_get_courses_db() {
        dotenv().ok();
        let mut conn = get_db_conn().await;

        let result = get_courses_db(&mut conn).await;

        assert!(result.is_ok());

        let courses = result.unwrap();
        assert!(!courses.is_empty());
    }

    #[actix_web::test]
    async fn test_get_course_by_id_db() {
        dotenv().ok();
        let mut conn = get_db_conn().await;

        let tutor = _create_new_tutor(&mut conn).await;

        let new_course_name: String = Name().fake();
        let new_course = NewCourseInput {
            tutor_id: tutor.id,
            name: new_course_name,
            description: Some("This is a test course".to_string()),
        };

        let created_course = post_course_db(&mut conn, new_course.clone()).await.unwrap();

        let found_course = get_course_by_id_db(&mut conn, &created_course.id.to_string()).await;

        assert!(found_course.is_ok());

        let found_course = found_course.unwrap();
        assert!(found_course.is_some());

        let found_course = found_course.unwrap();

        assert_eq!(found_course.name, created_course.name);
    }

    #[actix_web::test]
    async fn test_patch_course_by_id_db() {
        dotenv().ok();
        let mut conn = get_db_conn().await;

        let tutor = _create_new_tutor(&mut conn).await;

        let new_course_name: String = Name().fake();
        let new_course = NewCourseInput {
            tutor_id: tutor.id,
            name: new_course_name,
            description: Some("This is a test course".to_string()),
        };

        let created_course = post_course_db(&mut conn, new_course.clone()).await.unwrap();

        let tutor02 = _create_new_tutor(&mut conn).await;

        let update_data = UpdateCourseInput {
            tutor_id: Some(tutor02.id),
            name: Some("Updated Course Name".to_string()),
            description: Some("Updated Description".to_string()),
        };

        let result = patch_course_by_id_db(
            &mut conn,
            &created_course.id.to_string(),
            update_data.clone(),
        )
        .await;

        assert!(result.is_ok());

        let updated_course = get_course_by_id_db(&mut conn, &created_course.id.to_string()).await;
        assert!(updated_course.is_ok());

        let updated_course = updated_course.unwrap();
        assert!(updated_course.is_some());

        let updated_course = updated_course.unwrap();

        assert_eq!(updated_course.name, update_data.name.unwrap());
        assert_eq!(updated_course.tutor_id, update_data.tutor_id.unwrap());
    }

    #[actix_web::test]
    async fn test_delete_course_by_id_db() {
        dotenv().ok();
        let mut conn = get_db_conn().await;

        let tutor = _create_new_tutor(&mut conn).await;

        let new_course_name: String = Name().fake();
        let new_course = NewCourseInput {
            tutor_id: tutor.id,
            name: new_course_name,
            description: Some("This is a test course".to_string()),
        };

        let created_course = post_course_db(&mut conn, new_course.clone()).await.unwrap();

        let result = delete_course_by_id_db(&mut conn, &created_course.id.to_string()).await;

        assert!(result.is_ok());

        let deleted_course = get_course_by_id_db(&mut conn, &created_course.id.to_string()).await;
        assert!(deleted_course.is_ok());

        let deleted_course = deleted_course.unwrap();
        assert!(deleted_course.is_none());
    }

    #[actix_web::test]
    async fn test_get_course_by_tutor_id_db() {
        dotenv().ok();
        let mut conn = get_db_conn().await;

        let tutor = _create_new_tutor(&mut conn).await;

        let new_course_name: String = Name().fake();
        let new_course = NewCourseInput {
            tutor_id: tutor.id,
            name: new_course_name,
            description: Some("This is a test course".to_string()),
        };

        let created_course = post_course_db(&mut conn, new_course.clone()).await.unwrap();

        let result = get_courses_by_tutor_id_db(&mut conn, &tutor.id.to_string()).await;

        assert!(result.is_ok());

        let courses = result.unwrap();
        assert!(!courses.is_empty());

        let found_course = courses
            .iter()
            .find(|&course| course.id == created_course.id);
        assert!(found_course.is_some());
    }
}
