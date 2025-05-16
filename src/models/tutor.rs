use std::clone;

use actix_web::web::Json;
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

impl From<Json<NewTutorInput>> for NewTutorInput {
    fn from(value: Json<NewTutorInput>) -> Self {
        Self {
            name: value.name.clone(),
            age: value.age,
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct UpdateTurtorInput {
    pub name: Option<String>,
    pub age: Option<i32>,
}

impl From<Json<UpdateTurtorInput>> for UpdateTurtorInput {
    fn from(value: Json<UpdateTurtorInput>) -> Self {
        Self {
            name: {
                if let Some(name) = value.name.clone() {
                    Some(name)
                } else {
                    None
                }
            },
            age: {
                if let Some(age) = value.age {
                    Some(age)
                } else {
                    None
                }
            },
        }
    }
}
