use actix_web::http::StatusCode;
use actix_web::{HttpResponse, ResponseError};
use serde::Serialize;
use thiserror::Error;

use crate::ErrorResponse;

#[derive(Debug, Error, Serialize)]
pub enum ApiError {
    #[error("Validation failed: {0}")]
    ValidationError(String),
    #[error("Product not found")]
    NotFound,
    #[error("Failed to fetch products")]
    FetchError,
    #[error("Failed to create product")]
    DatabaseError,
    #[error("Failed to update product")]
    UpdateError,
    #[error("Failed to delete product")]
    DeleteError,
}

impl ResponseError for ApiError {
    fn error_response(&self) -> HttpResponse {
        let (status, message) = match self {
            ApiError::ValidationError(msg) => (StatusCode::BAD_REQUEST, msg.clone()),
            ApiError::NotFound => (StatusCode::NOT_FOUND, self.to_string()),
            _ => (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()),
        };

        HttpResponse::build(status).json(ErrorResponse {
            status: "error".to_string(),
            message,
            error: Some(self.to_string()),
        })
    }

    fn status_code(&self) -> StatusCode {
        match self {
            ApiError::ValidationError(_) => StatusCode::BAD_REQUEST,
            ApiError::NotFound => StatusCode::NOT_FOUND,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
