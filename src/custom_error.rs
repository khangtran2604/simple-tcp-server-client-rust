use std::{error::Error, fmt::Display};

use actix_web::{HttpResponse, ResponseError, http::StatusCode};
use serde::Serialize;

#[derive(Debug)]
pub enum CustomError {
    ActiveWebError(String),
    SqlxError(String),
    InvalidInputData(String),
}

impl Display for CustomError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CustomError::ActiveWebError(msg) => write!(f, "{}", msg),
            CustomError::SqlxError(msg) => write!(f, "{}", msg),
            CustomError::InvalidInputData(msg) => write!(f, "{}", msg),
        }
    }
}

impl Error for CustomError {}

#[derive(Serialize)]
struct CustomErrorResponse {
    error_message: String,
}

impl ResponseError for CustomError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        match self {
            CustomError::ActiveWebError(_) | CustomError::SqlxError(_) => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
            CustomError::InvalidInputData(_) => StatusCode::BAD_REQUEST,
        }
    }

    fn error_response(&self) -> actix_web::HttpResponse<actix_web::body::BoxBody> {
        match self {
            CustomError::ActiveWebError(e) | CustomError::SqlxError(e) => {
                HttpResponse::build(self.status_code()).json(CustomErrorResponse {
                    error_message: e.to_string(),
                })
            }
            CustomError::InvalidInputData(e) => {
                HttpResponse::build(self.status_code()).json(CustomErrorResponse {
                    error_message: e.to_string(),
                })
            }
        }
    }
}
