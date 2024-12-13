use crate::services::order_service::{
    CreateOrderRequest, OrderItemRequest, OrderService, UpdatePaymentStatusRequest,
};
use crate::utils::actix_error::ApiError;
use crate::ApiResponse;
use actix_web::{delete, get, post, put, web, HttpResponse};
use sea_orm::DatabaseConnection;
use validator::Validate;

#[post("/orders")]
async fn create_order_handler(
    db: web::Data<DatabaseConnection>,
    payload: web::Json<CreateOrderPayload>, // Expect JSON object
) -> Result<HttpResponse, ApiError> {
    let user_id = payload.user_id;

    // Fetch all carts for the user
    let carts = OrderService::get_all_carts_for_user(db.get_ref(), user_id)
        .await?;

    if carts.is_empty() {
        return Err(ApiError::ValidationError(
            "No carts found for the user".to_string(),
        ));
    }

    let mut all_cart_items = Vec::new();

    // Collect items from all carts
    for cart in carts {
        let cart_items = OrderService::get_cart_items_for_user(db.get_ref(), cart.id)
            .await?;

        all_cart_items.extend(cart_items);
    }

    if all_cart_items.is_empty() {
        return Err(ApiError::ValidationError(
            "No items in carts to create an order".to_string(),
        ));
    }

    // Map cart items to order items with prices
    let mut order_items = Vec::new();
    for item in all_cart_items {
        let price = OrderService::get_product_price(db.get_ref(), item.product_id)
            .await?;

        order_items.push(OrderItemRequest {
            product_id: item.product_id,
            quantity: item.quantity,
            price,
        });
    }

    // Prepare the order creation request
    let create_order_request = CreateOrderRequest {
        user_id,
        items: order_items,
    };

    // Validate the order creation request
    create_order_request
        .validate()
        .map_err(|e| ApiError::ValidationError(e.to_string()))?;

    // Create the order using the service
    let order = OrderService::create_order(db.get_ref(), create_order_request)
        .await?;

    // Clear all carts after the order is created
    OrderService::clear_all_carts(db.get_ref(), user_id)
        .await?;

    Ok(HttpResponse::Created().json(ApiResponse {
        status: "success".to_string(),
        message: "Order created successfully from all carts".to_string(),
        data: Some(order),
    }))
}

/// Handler to retrieve an order with its items
#[get("/orders/{order_id}")]
async fn get_order_with_items_handler(
    db: web::Data<DatabaseConnection>,
    order_id: web::Path<i32>,
) -> Result<HttpResponse, ApiError> {
    let (order, items) = OrderService::get_order_with_items(db.get_ref(), *order_id)
        .await?;

    Ok(HttpResponse::Ok().json(ApiResponse {
        status: "success".to_string(),
        message: "Order retrieved successfully".to_string(),
        data: Some((order, items)),
    }))
}

/// Handler to update the payment status of an order
#[put("/orders/{order_id}/payment-status")]
async fn update_payment_status_handler(
    db: web::Data<DatabaseConnection>,
    order_id: web::Path<i32>,
    request: web::Json<UpdatePaymentStatusRequest>,
) -> Result<HttpResponse, ApiError> {
    // Validate input
    request
        .validate()
        .map_err(|e| ApiError::ValidationError(e.to_string()))?;

    let updated_order =
        OrderService::update_payment_status(db.get_ref(), *order_id, request.into_inner())
            .await?;

    Ok(HttpResponse::Ok().json(ApiResponse {
        status: "success".to_string(),
        message: "Payment status updated successfully".to_string(),
        data: Some(updated_order),
    }))
}

/// Handler to delete an order and its items
#[delete("/orders/{order_id}")]
async fn delete_order_handler(
    db: web::Data<DatabaseConnection>,
    order_id: web::Path<i32>,
) -> Result<HttpResponse, ApiError> {
    OrderService::delete_order(db.get_ref(), *order_id)
        .await?;

    Ok(HttpResponse::Ok().json(ApiResponse::<String> {
        status: "success".to_string(),
        message: "Order deleted successfully".to_string(),
        data: None,
    }))
}

use serde::Deserialize;

#[derive(Deserialize)]
struct CreateOrderPayload {
    user_id: i32,
}
