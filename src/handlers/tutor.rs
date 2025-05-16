use actix_web::{
    HttpResponse,
    http::StatusCode,
    web::{Data, Json, Path},
};

use crate::{
    custom_error::{CustomError, CustomErrorResponse},
    db_access::tutor::{
        del_tutor_by_id_db, get_tutor_by_id_db, get_tutors_db, patch_tutor_by_id_db, post_tutor_db,
    },
    models::tutor::{NewTutorInput, UpdateTurtorInput},
    state::AppState,
};

pub async fn get_tutors(app_state: Data<AppState>) -> Result<HttpResponse, CustomError> {
    let mut conn = app_state.conn.lock().unwrap();

    get_tutors_db(&mut conn)
        .await
        .map(|tutors| HttpResponse::Ok().json(tutors))
}

pub async fn get_tutor_by_id(
    app_state: Data<AppState>,
    path: Path<i32>,
) -> Result<HttpResponse, CustomError> {
    let tutor_id = path.into_inner().to_string();
    let mut conn = app_state.conn.lock().unwrap();

    get_tutor_by_id_db(&mut conn, &tutor_id).await.map(|tutor| {
        if tutor.is_none() {
            return HttpResponse::build(StatusCode::NOT_FOUND).json(CustomErrorResponse {
                error_message: "Tutor was not found!".to_string(),
            });
        }

        HttpResponse::Ok().json(tutor)
    })
}

pub async fn post_tutor(
    new_tutor_data: Json<NewTutorInput>,
    app_state: Data<AppState>,
) -> Result<HttpResponse, CustomError> {
    let mut conn = app_state.conn.lock().unwrap();

    post_tutor_db(&mut conn, NewTutorInput::from(new_tutor_data))
        .await
        .map(|new_tutor| HttpResponse::Ok().json(new_tutor))
}

pub async fn patch_tutor_by_id(
    update_tutor_data: Json<UpdateTurtorInput>,
    app_state: Data<AppState>,
    path: Path<i32>,
) -> Result<HttpResponse, CustomError> {
    let tutor_id = path.into_inner().to_string();
    let mut conn = app_state.conn.lock().unwrap();

    patch_tutor_by_id_db(
        &mut conn,
        &tutor_id,
        UpdateTurtorInput::from(update_tutor_data),
    )
    .await
    .map(|_| HttpResponse::Ok().json("The tutor is updated successfully!"))
}

pub async fn delete_tutor_by_id(
    app_state: Data<AppState>,
    path: Path<i32>,
) -> Result<HttpResponse, CustomError> {
    let tutor_id = path.into_inner().to_string();
    let mut conn = app_state.conn.lock().unwrap();

    del_tutor_by_id_db(&mut conn, &tutor_id)
        .await
        .map(|_| HttpResponse::Ok().json("The tutor is deleted successfully!"))
}
