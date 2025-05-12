use serde::{Deserialize, Serialize};
use sqlx::types::chrono::{DateTime, Utc};

#[derive(Debug, Serialize)]
pub struct Tutor {
    pub id: i32,
    pub name: String,
    pub age: i32,
    pub create_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct NewTutorInput {
    pub name: String,
    pub age: i32,
}

#[derive(Debug, Deserialize)]
pub struct UpdateTurtorInput {
    pub name: Option<String>,
    pub age: Option<i32>,
}
