use crate::models::product::ProductRequest;
use crate::services::product_service; // Import the service where product logic resides
use crate::utils::actix_error::ApiError;
use actix_web::{delete, get, post, put, web, HttpResponse};
use sea_orm::DatabaseConnection;
use serde::{Deserialize, Serialize};
use validator::Validate;
/// Standard response format for success
#[derive(Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub status: String,
    pub message: String,
    pub data: Option<T>,
}

/// Standard error response format
#[derive(Serialize, Deserialize)]
pub struct ErrorResponse {
    pub status: String,
    pub message: String,
    pub error: Option<String>,
}

#[post("/products")]
pub async fn create_product(
    db: web::Data<DatabaseConnection>,
    request: web::Json<ProductRequest>,
) -> Result<HttpResponse, ApiError> {
    request
        .validate()?;

    let product_response = product_service::create_product(db.get_ref(), request.into_inner())
        .await?;

    Ok(HttpResponse::Created().json(ApiResponse {
        status: "success".to_string(),
        message: "Product created successfully".to_string(),
        data: Some(product_response),
    }))
}

#[get("/products")]
pub async fn get_all_products(db: web::Data<DatabaseConnection>) -> Result<HttpResponse, ApiError> {
    let product_responses = product_service::get_all_products(db.get_ref())
        .await?;

    Ok(HttpResponse::Ok().json(ApiResponse {
        status: "success".to_string(),
        message: "Products fetched successfully".to_string(),
        data: Some(product_responses),
    }))
}

#[get("/products/{id}")]
pub async fn get_product(
    db: web::Data<DatabaseConnection>,
    product_id: web::Path<i32>,
) -> Result<HttpResponse, ApiError> {
    let product_response = product_service::get_product_by_id(db.get_ref(), *product_id)
        .await?;

    Ok(HttpResponse::Ok().json(ApiResponse {
        status: "success".to_string(),
        message: "Product fetched successfully".to_string(),
        data: Some(product_response),
    }))
}

#[put("/products/{id}")]
pub async fn update_product(
    db: web::Data<DatabaseConnection>,
    product_id: web::Path<i32>,
    request: web::Json<ProductRequest>,
) -> Result<HttpResponse, ApiError> {
    request
        .validate()?;

    let product_response =
        product_service::update_product(db.get_ref(), *product_id, request.into_inner())
            .await?;

    Ok(HttpResponse::Ok().json(ApiResponse {
        status: "success".to_string(),
        message: "Product updated successfully".to_string(),
        data: Some(product_response),
    }))
}

#[delete("/products/{id}")]
pub async fn delete_product(
    db: web::Data<DatabaseConnection>,
    product_id: web::Path<i32>,
) -> Result<HttpResponse, ApiError> {
    product_service::delete_product(db.get_ref(), *product_id)
        .await?;

    Ok(HttpResponse::Ok().json(ApiResponse::<()> {
        status: "success".to_string(),
        message: "Product deleted successfully".to_string(),
        data: None,
    }))
}
