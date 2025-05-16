use actix_web::web::{self, ServiceConfig, delete, get, patch, post};

use crate::handlers::tutor::{
    delete_tutor_by_id, get_tutor_by_id, get_tutors, patch_tutor_by_id, post_tutor,
};

pub fn tutor_routes(cfg: &mut ServiceConfig) {
    cfg.service(
        web::scope("/tutors")
            .route("", post().to(post_tutor))
            .route("", get().to(get_tutors))
            .route("/{tutor_id}", get().to(get_tutor_by_id))
            .route("/{tutor_id}", patch().to(patch_tutor_by_id))
            .route("/{tutor_id}", delete().to(delete_tutor_by_id)),
    );
}
