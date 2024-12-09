use actix_web::http::StatusCode;
use actix_web::{HttpResponse, ResponseError};
use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error, Serialize)]
pub enum ApiError {
    #[error("Validation failed: {0}")]
    ValidationError(String),
    #[error("{entity} not found")]
    NotFound { entity: String },
    #[error("Failed to fetch {entity}")]
    FetchError { entity: String },
    #[error("Failed to create {entity}")]
    DatabaseError { entity: String },
    #[error("Failed to update {entity}")]
    UpdateError { entity: String },
    #[error("Failed to delete {entity}")]
    DeleteError { entity: String },
}

impl ResponseError for ApiError {
    fn error_response(&self) -> HttpResponse {
        let (status, message) = match self {
            ApiError::ValidationError(msg) => (StatusCode::BAD_REQUEST, msg.clone()),
            ApiError::NotFound { entity } => (StatusCode::NOT_FOUND, format!("{} not found", entity)),
            ApiError::FetchError { entity } => (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to fetch {}", entity)),
            ApiError::DatabaseError { entity } => (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to create {}", entity)),
            ApiError::UpdateError { entity } => (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to update {}", entity)),
            ApiError::DeleteError { entity } => (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to delete {}", entity)),
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
            ApiError::NotFound { .. } => StatusCode::NOT_FOUND,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

#[derive(Serialize)]
pub struct ErrorResponse {
    pub status: String,
    pub message: String,
    pub error: Option<String>,
}
