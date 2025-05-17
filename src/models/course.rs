use actix_web::web::Json;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, PartialEq, sqlx::FromRow)]
pub struct Course {
    pub id: i32,
    pub tutor_id: i32,
    pub name: String,
    pub description: String,
    pub posted_time: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct NewCourseInput {
    pub tutor_id: i32,
    pub name: String,
    pub description: Option<String>,
}

impl NewCourseInput {
    pub fn new(tutor_id: i32, name: String, description: Option<String>) -> Self {
        Self {
            tutor_id,
            name,
            description,
        }
    }
}

impl From<Json<NewCourseInput>> for NewCourseInput {
    fn from(value: Json<NewCourseInput>) -> Self {
        Self {
            tutor_id: value.tutor_id,
            name: value.name.clone(),
            description: value.description.clone(),
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct UpdateCourseInput {
    pub tutor_id: Option<i32>,
    pub name: Option<String>,
    pub description: Option<String>,
}

impl UpdateCourseInput {
    pub fn new(tutor_id: Option<i32>, name: Option<String>, description: Option<String>) -> Self {
        Self {
            tutor_id,
            name,
            description,
        }
    }
}

impl From<Json<UpdateCourseInput>> for UpdateCourseInput {
    fn from(value: Json<UpdateCourseInput>) -> Self {
        Self {
            tutor_id: value.tutor_id,
            name: value.name.clone(),
            description: value.description.clone(),
        }
    }
}
