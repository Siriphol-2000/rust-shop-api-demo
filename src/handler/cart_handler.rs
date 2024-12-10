use crate::services::cart_service;
use crate::services::cart_service::{CartItemRequest, CartItemResponse, CartRequest, CartResponse};
use crate::utils::actix_error::ApiError;
use crate::ApiResponse;
use actix_web::{delete, post, web, HttpResponse};
use sea_orm::DatabaseConnection;
use validator::Validate;

#[post("/carts")]
async fn create_cart_handler(
    db: web::Data<DatabaseConnection>,
    request: web::Json<CartRequest>,
) -> Result<HttpResponse, ApiError> {
    // Validate input
    request
        .validate()
        .map_err(|e| ApiError::ValidationError(e.to_string()))?;

    // Create cart using the service
    let cart_response = cart_service::create_cart(db.get_ref(), request.into_inner())
        .await
        .map_err(|_| ApiError::DatabaseError {
            entity: "cart".to_string(),
        })?;

    Ok(HttpResponse::Created().json(ApiResponse::<CartResponse> {
        status: "success".to_string(),
        message: "Cart created successfully".to_string(),
        data: Some(cart_response),
    }))
}

#[post("/carts/{cart_id}/items")]
async fn add_item_to_cart_handler(
    db: web::Data<DatabaseConnection>,
    cart_id: web::Path<i32>,
    request: web::Json<CartItemRequest>,
) -> Result<HttpResponse, ApiError> {
    // Validate input
    request
        .validate()
        .map_err(|e| ApiError::ValidationError(e.to_string()))?;

    // Add item to cart using the service
    let cart_item_response =
        cart_service::add_item_to_cart(db.get_ref(), *cart_id, request.into_inner())
            .await
            .map_err(|_| ApiError::DatabaseError {
                entity: "cart_item".to_string(),
            })?;

    Ok(
        HttpResponse::Created().json(ApiResponse::<CartItemResponse> {
            status: "success".to_string(),
            message: "Item added to cart successfully".to_string(),
            data: Some(cart_item_response),
        }),
    )
}

#[delete("/carts/items/{cart_item_id}")]
async fn remove_item_from_cart_handler(
    db: web::Data<DatabaseConnection>,
    cart_item_id: web::Path<i32>,
) -> Result<HttpResponse, ApiError> {
    // Remove item from cart using the service
    cart_service::remove_item_from_cart(db.get_ref(), *cart_item_id)
        .await
        .map_err(|_| ApiError::DatabaseError {
            entity: "cart_item".to_string(),
        })?;

    Ok(HttpResponse::Ok().json(ApiResponse::<String> {
        status: "success".to_string(),
        message: "Item removed from cart successfully".to_string(),
        data: None,
    }))
}

#[delete("/carts/{cart_id}/clear")]
async fn clear_cart_handler(
    db: web::Data<DatabaseConnection>,
    cart_id: web::Path<i32>,
) -> Result<HttpResponse, ApiError> {
    // Clear all items from cart using the service
    cart_service::clear_cart(db.get_ref(), *cart_id)
        .await
        .map_err(|_| ApiError::DatabaseError {
            entity: "cart_item".to_string(),
        })?;

    Ok(HttpResponse::Ok().json(ApiResponse::<String> {
        status: "success".to_string(),
        message: "Cart cleared successfully".to_string(),
        data: None,
    }))
}
