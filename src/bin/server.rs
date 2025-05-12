use std::{
    borrow::BorrowMut,
    ops::{Deref, DerefMut},
};

use actix_web::web::Data;
use db_access::get_db_conn;
use dotenv::dotenv;
use seeds::{course::create_course_table, tutor::create_tutor_table};
use state::AppState;

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
async fn main() {
    dotenv().ok();

    let app_data = AppState::init().await;
    let shared_data = Data::new(app_data);

    let mut conn = shared_data.conn.lock().unwrap();
    create_tutor_table(&mut conn).await;
    create_course_table(&mut conn).await;
}
