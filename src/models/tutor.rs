use std::clone;

use serde::{Deserialize, Serialize};
use sqlx::types::chrono::{DateTime, Utc};

#[derive(Debug, Serialize, sqlx::FromRow, PartialEq)]
pub struct Tutor {
    pub id: i32,
    pub name: String,
    pub age: i32,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct NewTutorInput {
    pub name: String,
    pub age: i32,
}

impl NewTutorInput {
    pub fn new(name: String, age: i32) -> Self {
        Self { name, age }
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct UpdateTurtorInput {
    pub name: Option<String>,
    pub age: Option<i32>,
}
