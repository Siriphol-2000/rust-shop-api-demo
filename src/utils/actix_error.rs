use actix_web::{error, http::StatusCode, HttpResponse};
use derive_more::{Display, From};
use sea_orm::DbErr;
use serde::Serialize;
use validator::ValidationErrors;

use crate::services::service_error::ServiceError;

#[derive(Debug, Display)]
pub enum ApiError {
    DatabaseError(String),
    NotFound(String),
    AuthenticationError(String),
    InternalServerError(String),
    // Manually handle ValidationError, do not derive From for String
    ValidationError(String),
}

// Implement From<DbErr> for ApiError
impl From<DbErr> for ApiError {
    fn from(err: DbErr) -> Self {
        match err {
            DbErr::RecordNotFound(msg) => ApiError::NotFound(msg),
            _ => ApiError::DatabaseError(err.to_string()),
        }
    }
}

// Implement From<ServiceError> for ApiError
impl From<ServiceError> for ApiError {
    fn from(err: ServiceError) -> Self {
        match err {
            ServiceError::Database(e) => ApiError::DatabaseError(e.to_string()),
            _ => ApiError::NotFound("".into()),
        }
    }
}

// Manually implement From<ValidationErrors> for ApiError
impl From<ValidationErrors> for ApiError {
    fn from(err: ValidationErrors) -> Self {
        // Create a string representation of the validation errors
        let error_string = err
            .field_errors()
            .into_iter()
            .map(|(field, errors)| {
                format!(
                    "Field '{}': {}",
                    field,
                    errors
                        .iter()
                        .map(|e| e
                            .message
                            .clone()
                            .unwrap_or_else(|| "Unknown error".into())
                            .to_string()) // Convert Cow to String
                        .collect::<Vec<String>>()
                        .join(", ")
                )
            })
            .collect::<Vec<String>>()
            .join("; ");

        // Return the ApiError with the generated string
        ApiError::ValidationError(error_string)
    }
}

// Implement the ResponseError trait for ApiError
impl error::ResponseError for ApiError {
    fn error_response(&self) -> HttpResponse {
        let error_response = match self {
            ApiError::DatabaseError(msg) => ErrorResponse {
                error: "Database error".to_string(),
                message: msg.clone(),
            },
            ApiError::ValidationError(msg) => ErrorResponse {
                error: "Validation error".to_string(),
                message: msg.clone(),
            },
            ApiError::NotFound(msg) => ErrorResponse {
                error: "Not found".to_string(),
                message: msg.clone(),
            },
            ApiError::AuthenticationError(msg) => ErrorResponse {
                error: "Authentication error".to_string(),
                message: msg.clone(),
            },
            ApiError::InternalServerError(_) => ErrorResponse {
                error: "Internal server error".to_string(),
                message: "An unexpected error occurred".to_string(),
            },
        };

        HttpResponse::build(self.status_code()).json(error_response)
    }

    fn status_code(&self) -> StatusCode {
        match self {
            ApiError::DatabaseError(_) => StatusCode::BAD_REQUEST,
            ApiError::ValidationError(_) => StatusCode::BAD_REQUEST,
            ApiError::NotFound(_) => StatusCode::NOT_FOUND,
            ApiError::AuthenticationError(_) => StatusCode::UNAUTHORIZED,
            ApiError::InternalServerError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
    message: String,
}
