use crate::models::product::{ProductRequest, ProductResponse};
use crate::services::product_service;  // Import the service where product logic resides
use actix_web::{get, post, put, delete, web, HttpResponse, Responder};
use sea_orm::DatabaseConnection;
use validator::Validate;
use serde::{Serialize, Deserialize};

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
async fn create_product(
    db: web::Data<DatabaseConnection>,
    request: web::Json<ProductRequest>,  // Deserialize request body
) -> impl Responder {
    // Validate the input
    if let Err(validation_errors) = request.validate() {
        return HttpResponse::BadRequest().json(ErrorResponse {
            status: "error".to_string(),
            message: "Validation failed".to_string(),
            error: Some(validation_errors.to_string()),
        });
    }

    // Call the service to create the product
    match product_service::create_product(db.get_ref(), request.into_inner()).await {
        Ok(product_response) => HttpResponse::Created().json(ApiResponse {
            status: "success".to_string(),
            message: "Product created successfully".to_string(),
            data: Some(product_response),
        }),
        Err(err) => HttpResponse::InternalServerError().json(ErrorResponse {
            status: "error".to_string(),
            message: "Failed to create product".to_string(),
            error: Some(err),
        }),
    }
}
#[get("/products")]
async fn get_all_products(db: web::Data<DatabaseConnection>) -> impl Responder {
    // Call the service to fetch all products
    match product_service::get_all_products(db.get_ref()).await {
        Ok(product_responses) => HttpResponse::Ok().json(ApiResponse {
            status: "success".to_string(),
            message: "Products found".to_string(),
            data: Some(product_responses),
        }),
        Err(err) => HttpResponse::InternalServerError().json(ErrorResponse {
            status: "error".to_string(),
            message: "Failed to fetch products".to_string(),
            error: Some(err),
        }),
    }
}

#[get("/products/{id}")]
async fn get_product(
    db: web::Data<DatabaseConnection>,
    product_id: web::Path<i32>,  // Extract `id` from the URL
) -> impl Responder {
    // Call the service to fetch product by ID
    match product_service::get_product_by_id(db.get_ref(), *product_id).await {
        Ok(product_response) => HttpResponse::Ok().json(ApiResponse {
            status: "success".to_string(),
            message: "Product found".to_string(),
            data: Some(product_response),
        }),
        Err(err) => HttpResponse::NotFound().json(ErrorResponse {
            status: "error".to_string(),
            message: "Product not found".to_string(),
            error: Some(err),
        }),
    }
}

#[put("/products/{id}")]
async fn update_product(
    db: web::Data<DatabaseConnection>,
    product_id: web::Path<i32>,  // Extract `id` from the URL
    request: web::Json<ProductRequest>,  // Deserialize the updated product data
) -> impl Responder {
    // Validate the input
    if let Err(validation_errors) = request.validate() {
        return HttpResponse::BadRequest().json(ErrorResponse {
            status: "error".to_string(),
            message: "Validation failed".to_string(),
            error: Some(validation_errors.to_string()),
        });
    }

    // Call the service to update the product
    match product_service::update_product(db.get_ref(), *product_id, request.into_inner()).await {
        Ok(product_response) => HttpResponse::Ok().json(ApiResponse {
            status: "success".to_string(),
            message: "Product updated successfully".to_string(),
            data: Some(product_response),
        }),
        Err(err) => HttpResponse::InternalServerError().json(ErrorResponse {
            status: "error".to_string(),
            message: "Failed to update product".to_string(),
            error: Some(err),
        }),
    }
}

#[delete("/products/{id}")]
async fn delete_product(
    db: web::Data<DatabaseConnection>,
    product_id: web::Path<i32>,  // Extract `id` from the URL
) -> impl Responder {
    // Call the service to delete the product
    match product_service::delete_product(db.get_ref(), *product_id).await {
        Ok(()) => HttpResponse::Ok().json(ApiResponse::<()>{ 
            status: "success".to_string(),
            message: "Product deleted successfully".to_string(),
            data: None,
        }),
        Err(err) => HttpResponse::InternalServerError().json(ErrorResponse {
            status: "error".to_string(),
            message: "Failed to delete product".to_string(),
            error: Some(err),
        }),
    }
}

