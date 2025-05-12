use std::sync::Mutex;

use sqlx::SqliteConnection;

use crate::db_access::get_db_conn;

pub struct AppState {
    pub conn: Mutex<SqliteConnection>,
}

impl AppState {
    pub async fn init() -> Self {
        let conn = get_db_conn().await;

        Self {
            conn: Mutex::new(conn),
        }
    }
}
