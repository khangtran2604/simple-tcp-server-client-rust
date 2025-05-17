use actix_web::web::{self, ServiceConfig, delete, get, patch, post};

use crate::handlers::course::{
    delete_course_by_id, get_course_by_id, get_course_by_tutor_id, get_courses, patch_course_by_id,
    post_course,
};

pub fn course_routes(cfg: &mut ServiceConfig) {
    cfg.service(
        web::scope("/courses")
            .route("", post().to(post_course))
            .route("", get().to(get_courses))
            .route("/tutor/{tutor_id}", get().to(get_course_by_tutor_id))
            .route("/{course_id}", get().to(get_course_by_id))
            .route("/{course_id}", patch().to(patch_course_by_id))
            .route("/{course_id}", delete().to(delete_course_by_id)),
    );
}
