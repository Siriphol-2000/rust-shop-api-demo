use crate::entities::{cart, cart_item};
use actix_web::{web, HttpResponse, Responder};
use chrono::Utc;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};
use serde::{Deserialize, Serialize};
use validator::Validate;

/// Struct for creating or updating a cart
#[derive(Debug, Deserialize, Validate)]
pub struct CartRequest {
    #[validate(range(min = 1))]
    pub user_id: i32,
}

/// Struct for creating or updating a cart item
#[derive(Debug, Deserialize, Validate)]
pub struct CartItemRequest {
    #[validate(range(min = 1))]
    pub product_id: i32,

    #[validate(range(min = 1, message = "Quantity must be at least 1"))]
    pub quantity: i32,
}

/// Struct used to return cart details in the response
#[derive(Debug, Serialize)]
pub struct CartResponse {
    pub id: i32,
    pub user_id: i32,
    pub created_at: String,
    pub updated_at: String,
}

/// Struct used to return cart item details in the response
#[derive(Debug, Serialize)]
pub struct CartItemResponse {
    pub id: i32,
    pub cart_id: i32,
    pub product_id: i32,
    pub quantity: i32,
    pub created_at: String,
    pub updated_at: String,
}

impl From<crate::entities::cart::Model> for CartResponse {
    fn from(model: crate::entities::cart::Model) -> Self {
        CartResponse {
            id: model.id,
            user_id: model.user_id,
            created_at: model.created_at.to_string(),
            updated_at: model.updated_at.to_string(),
        }
    }
}

impl From<crate::entities::cart_item::Model> for CartItemResponse {
    fn from(model: crate::entities::cart_item::Model) -> Self {
        CartItemResponse {
            id: model.id,
            cart_id: model.cart_id,
            product_id: model.product_id,
            quantity: model.quantity,
            created_at: model.created_at.to_string(),
            updated_at: model.updated_at.to_string(),
        }
    }
}

// /// Custom validator to check that a quantity is greater than 0.
// fn validate_quantity(value: &i32) -> Result<(), ValidationError> {
//     if *value < 1 {
//         return Err(ValidationError::new("quantity_less_than_one"));
//     }
//     Ok(())
// }

/// Service function to create a new cart
pub async fn create_cart(
    db: &DatabaseConnection,
    request: CartRequest,
) -> Result<CartResponse, String> {
    // Get the current timestamp
    let now_utc = Utc::now();
    let now_fixed: chrono::DateTime<chrono::FixedOffset> = now_utc.into(); // Convert to FixedOffset
                                                                           // Validate the input
    if let Err(validation_errors) = request.validate() {
        return Err(format!("Validation failed: {}", validation_errors));
    }

    // Create the cart in the database
    let cart = cart::ActiveModel {
        user_id: sea_orm::Set(request.user_id),
        created_at: Set(now_fixed), // Set the created_at timestamp
        updated_at: Set(now_fixed), // Set the updated_at timestamp
        ..Default::default()
    };

    let inserted_cart = cart
        .insert(db)
        .await
        .map_err(|err| format!("Failed to create cart: {}", err))?;

    // Return the response with cart details
    Ok(CartResponse::from(inserted_cart))
}

/// Service function to add an item to the cart
pub async fn add_item_to_cart(
    db: &DatabaseConnection,
    cart_id: i32,
    request: CartItemRequest,
) -> Result<CartItemResponse, String> {
    // Get the current timestamp
    let now_utc = Utc::now();
    let now_fixed: chrono::DateTime<chrono::FixedOffset> = now_utc.into(); // Convert to FixedOffset
                                                                           // Validate the input
    if let Err(validation_errors) = request.validate() {
        return Err(format!("Validation failed: {}", validation_errors));
    }

    // Create the cart item in the database
    let cart_item = cart_item::ActiveModel {
        cart_id: sea_orm::Set(cart_id),
        product_id: sea_orm::Set(request.product_id),
        quantity: sea_orm::Set(request.quantity),
        created_at: Set(now_fixed), // Set the created_at timestamp
        updated_at: Set(now_fixed), // Set the updated_at timestamp
        ..Default::default()
    };

    let inserted_cart_item = cart_item
        .insert(db)
        .await
        .map_err(|err| format!("Failed to add item to cart: {}", err))?;

    // Return the response with cart item details
    Ok(CartItemResponse::from(inserted_cart_item))
}

/// Service function to remove an item from the cart
pub async fn remove_item_from_cart(
    db: &DatabaseConnection,
    cart_item_id: i32,
) -> Result<(), String> {
    // Attempt to delete the cart item from the database
    cart_item::Entity::delete_many()
        .filter(cart_item::Column::Id.eq(cart_item_id)) // Apply filter to delete specific cart item
        .exec(db)
        .await
        .map_err(|err| format!("Failed to remove item from cart: {}", err))?;

    Ok(())
}

/// Service function to clear all items in a cart
pub async fn clear_cart(db: &DatabaseConnection, cart_id: i32) -> Result<(), String> {
    // Attempt to delete all items in the cart
    cart_item::Entity::delete_many() // Use delete_many instead of delete
        .filter(cart_item::Column::CartId.eq(cart_id))
        .exec(db)
        .await
        .map_err(|err| format!("Failed to clear cart: {}", err))?;

    Ok(())
}

/// Cart handler to create a new cart
#[actix_web::post("/carts")]
async fn create_cart_handler(
    db: web::Data<DatabaseConnection>,
    request: web::Json<CartRequest>,
) -> impl Responder {
    match create_cart(db.get_ref(), request.into_inner()).await {
        Ok(cart_response) => HttpResponse::Created().json(cart_response),
        Err(err) => HttpResponse::BadRequest().json(format!("Error: {}", err)),
    }
}

/// Cart item handler to add an item to the cart
#[actix_web::post("/carts/{cart_id}/items")]
async fn add_item_to_cart_handler(
    db: web::Data<DatabaseConnection>,
    cart_id: web::Path<i32>,
    request: web::Json<CartItemRequest>,
) -> impl Responder {
    match add_item_to_cart(db.get_ref(), *cart_id, request.into_inner()).await {
        Ok(cart_item_response) => HttpResponse::Created().json(cart_item_response),
        Err(err) => HttpResponse::BadRequest().json(format!("Error: {}", err)),
    }
}

/// Cart item handler to remove an item from the cart
#[actix_web::delete("/carts/items/{cart_item_id}")]
async fn remove_item_from_cart_handler(
    db: web::Data<DatabaseConnection>,
    cart_item_id: web::Path<i32>,
) -> impl Responder {
    match remove_item_from_cart(db.get_ref(), *cart_item_id).await {
        Ok(()) => HttpResponse::Ok().json("Item removed from cart"),
        Err(err) => HttpResponse::BadRequest().json(format!("Error: {}", err)),
    }
}

/// Cart handler to clear all items in the cart
#[actix_web::delete("/carts/{cart_id}/clear")]
async fn clear_cart_handler(
    db: web::Data<DatabaseConnection>,
    cart_id: web::Path<i32>,
) -> impl Responder {
    match clear_cart(db.get_ref(), *cart_id).await {
        Ok(()) => HttpResponse::Ok().json("Cart cleared"),
        Err(err) => HttpResponse::BadRequest().json(format!("Error: {}", err)),
    }
}
