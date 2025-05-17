use actix_web::{
    HttpResponse,
    web::{Data, Json, Path},
};

use crate::{
    custom_error::{CustomError, CustomErrorResponse},
    db_access::course::{
        delete_course_by_id_db, get_course_by_id_db, get_courses_by_tutor_id_db, get_courses_db,
        patch_course_by_id_db, post_course_db,
    },
    models::course::{NewCourseInput, UpdateCourseInput},
    state::AppState,
};

pub async fn get_course_by_tutor_id(
    app_state: Data<AppState>,
    path: Path<i32>,
) -> Result<HttpResponse, CustomError> {
    let tutor_id = path.into_inner().to_string();
    let mut conn = app_state.conn.lock().unwrap();

    get_courses_by_tutor_id_db(&mut conn, &tutor_id)
        .await
        .map(|course| HttpResponse::Ok().json(course))
}

pub async fn get_course_by_id(
    app_state: Data<AppState>,
    path: Path<i32>,
) -> Result<HttpResponse, CustomError> {
    let course_id = path.into_inner().to_string();
    let mut conn = app_state.conn.lock().unwrap();

    get_course_by_id_db(&mut conn, &course_id)
        .await
        .map(|course| {
            if course.is_none() {
                return HttpResponse::build(actix_web::http::StatusCode::NOT_FOUND).json(
                    CustomErrorResponse {
                        error_message: "Course was not found!".to_string(),
                    },
                );
            }

            HttpResponse::Ok().json(course)
        })
}

pub async fn get_courses(app_state: Data<AppState>) -> Result<HttpResponse, CustomError> {
    let mut conn = app_state.conn.lock().unwrap();

    get_courses_db(&mut conn)
        .await
        .map(|courses| HttpResponse::Ok().json(courses))
}

pub async fn post_course(
    course_data: Json<NewCourseInput>,
    app_state: Data<AppState>,
) -> Result<HttpResponse, CustomError> {
    let mut conn = app_state.conn.lock().unwrap();

    post_course_db(&mut conn, NewCourseInput::from(course_data))
        .await
        .map(|new_course| HttpResponse::Ok().json(new_course))
}

pub async fn patch_course_by_id(
    update_course_data: Json<UpdateCourseInput>,
    app_state: Data<AppState>,
    path: Path<i32>,
) -> Result<HttpResponse, CustomError> {
    let course_id = path.into_inner().to_string();
    let mut conn = app_state.conn.lock().unwrap();

    patch_course_by_id_db(
        &mut conn,
        &course_id,
        UpdateCourseInput::from(update_course_data),
    )
    .await
    .map(|_| HttpResponse::Ok().json("Course updated successfully"))
}

pub async fn delete_course_by_id(
    app_state: Data<AppState>,
    path: Path<i32>,
) -> Result<HttpResponse, CustomError> {
    let course_id = path.into_inner().to_string();
    let mut conn = app_state.conn.lock().unwrap();

    delete_course_by_id_db(&mut conn, &course_id)
        .await
        .map(|_| HttpResponse::Ok().json("Course deleted successfully"))
}
