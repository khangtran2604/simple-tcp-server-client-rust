use std::env;

use sqlx::{Connection, SqliteConnection};

pub mod course;
pub mod tutor;

pub async fn get_db_conn() -> SqliteConnection {
    let db_path = env::var("DB_PATH").expect("DB_PATH env should be set before!");

    SqliteConnection::connect(&db_path).await.unwrap()
}
