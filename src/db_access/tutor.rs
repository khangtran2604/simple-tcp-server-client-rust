use sqlx::SqliteConnection;

use crate::models::tutor::NewTutorInput;

pub fn post_tutor_db(&mut conn: SqliteConnection, data: NewTutorInput) {}
