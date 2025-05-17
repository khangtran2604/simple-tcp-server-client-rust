use actix_web::{
    App, HttpServer,
    web::{Data, JsonConfig},
};
use custom_error::CustomError;
use dotenv::dotenv;
use routes::{course::course_routes, tutor::tutor_routes};
use seeds::{course::create_course_table, tutor::create_tutor_table};
use state::AppState;
use std::io::Result;

#[path = "../handlers/mod.rs"]
mod handlers;

#[path = "../routes/mod.rs"]
mod routes;

#[path = "../models/mod.rs"]
mod models;

#[path = "../db_access/mod.rs"]
mod db_access;

#[path = "../seeds/mod.rs"]
mod seeds;

#[path = "../custom_error.rs"]
mod custom_error;

#[path = "../state.rs"]
mod state;

#[actix_web::main]
async fn main() -> Result<()> {
    dotenv().ok();

    let app_data = AppState::init().await;
    let shared_data = Data::new(app_data);

    // Seedings
    {
        let mut conn = shared_data.conn.lock().unwrap();
        create_tutor_table(&mut conn).await;
        create_course_table(&mut conn).await;
        // MutexGuard is dropped here at the end of this scope
    }

    println!("🚀 Server running on http://localhost:4000");

    HttpServer::new(move || {
        App::new()
            .app_data(shared_data.clone())
            .app_data(JsonConfig::default().error_handler(|_err, _req| {
                CustomError::InvalidInputData("Please provide invalid JSON input!".to_string())
                    .into()
            }))
            .configure(tutor_routes)
            .configure(course_routes)
    })
    .bind(("127.0.0.1", 4000))?
    .run()
    .await
}
