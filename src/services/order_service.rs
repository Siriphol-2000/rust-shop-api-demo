use super::cart_service::{CartItemResponse, CartResponse};
use crate::entities::{cart, cart_item, order, order_item};
use crate::utils::actix_error::ApiError;
use crate::utils::prompt_pay::PromptPayUtils; // Import the PromptPay utility
use chrono::Utc;
use dotenvy::dotenv;
use rust_decimal::prelude::ToPrimitive;
use rust_decimal::Decimal;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, DbErr, EntityTrait, QueryFilter, Set,
    TransactionTrait,
};
use serde::{Deserialize, Serialize};
use std::env;
use validator::{Validate, ValidationError};

/// Struct for creating an order
#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct CreateOrderRequest {
    #[validate(range(min = 1, message = "User ID must be at least 1"))]
    pub user_id: i32,

    #[validate(length(min = 1, message = "Order must contain at least one item"))]
    pub items: Vec<OrderItemRequest>,
}

/// Struct representing an item in an order
#[derive(Debug, Serialize, Deserialize)]
pub struct OrderItem {
    pub product_id: i32,
    pub quantity: i32,
    pub price: f64, // You might also want to store the price at the time of the order
    pub total: f64, // This can be derived as quantity * price
}

/// Struct for individual order items in the create request
#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct OrderItemRequest {
    #[validate(range(min = 1, message = "Product ID must be at least 1"))]
    pub product_id: i32,

    #[validate(range(min = 1, message = "Quantity must be at least 1"))]
    pub quantity: i32,

    #[validate(custom(
        function = "validate_price",
        message = "Price must be a positive value"
    ))]
    pub price: Decimal,
}

/// Struct for updating payment status
#[derive(Debug, Deserialize, Validate)]
pub struct UpdatePaymentStatusRequest {
    #[validate(length(min = 1, message = "Payment status cannot be empty"))]
    pub new_status: String,
}

/// Custom validator for price to ensure it is positive
fn validate_price(price: &Decimal) -> Result<(), ValidationError> {
    if *price <= Decimal::ZERO {
        return Err(ValidationError::new("price_must_be_positive"));
    }
    Ok(())
}

/// Custom Order model structure
#[derive(Debug, Deserialize, Serialize)]
pub struct OrderModel {
    pub id: i32,
    pub user_id: i32,
    pub total_amount: Decimal,
    pub payment_status: String,
    pub created_at: String,
    pub updated_at: String,
}

impl OrderModel {
    fn new(id: i32, user_id: i32, total_amount: Decimal, payment_status: String) -> Self {
        OrderModel {
            id,
            user_id,
            total_amount,
            payment_status,
            created_at: Utc::now().to_string(),
            updated_at: Utc::now().to_string(),
        }
    }
}

/// Custom Order Item model structure
#[derive(Debug, Deserialize, Serialize)]
pub struct OrderItemModel {
    pub id: i32,
    pub order_id: i32,
    pub product_id: i32,
    pub quantity: i32,
    pub price: Decimal,
    pub created_at: String,
    pub updated_at: String,
}

impl OrderItemModel {
    fn new(id: i32, order_id: i32, product_id: i32, quantity: i32, price: Decimal) -> Self {
        OrderItemModel {
            id,
            order_id,
            product_id,
            quantity,
            price,
            created_at: Utc::now().to_string(),
            updated_at: Utc::now().to_string(),
        }
    }
}

/// Service struct for managing orders
pub struct OrderService;

impl OrderService {
    /// Creates a new order along with its items
    pub async fn create_order(
        db: &DatabaseConnection,
        request: CreateOrderRequest,
    ) -> Result<OrderModel, DbErr> {
        // Validate the input
        request
            .validate()
            .map_err(|err| DbErr::Custom(format!("Validation failed: {}", err)))?;

        let total_amount: Decimal = request
            .items
            .iter()
            .map(|item| Decimal::from(item.quantity) * item.price)
            .sum();

        let transaction = db.begin().await?;

        // Create the order
        let new_order = order::ActiveModel {
            user_id: Set(request.user_id),
            total_amount: Set(total_amount),
            payment_status: Set("Pending".to_owned()),
            created_at: Set(chrono::Utc::now().into()),
            updated_at: Set(chrono::Utc::now().into()),
            ..Default::default()
        }
        .insert(&transaction)
        .await?;

        let order_model = OrderModel::new(
            new_order.id,
            request.user_id,
            total_amount,
            "Pending".to_owned(),
        );

        // Create order items
        for item in request.items {
            let new_order_item = order_item::ActiveModel {
                order_id: Set(new_order.id),
                product_id: Set(item.product_id),
                quantity: Set(item.quantity),
                price: Set(item.price),
                created_at: Set(chrono::Utc::now().into()),
                updated_at: Set(chrono::Utc::now().into()),
                ..Default::default()
            };

            new_order_item.insert(&transaction).await?;
        }

        transaction.commit().await?;
        dotenv().ok();
        // Generate PromptPay QR Code after successful order creation
        let phone_number: String = env::var("My_PHONE_NUMBER").expect("My_PHONE_NUMBER not set");
        let qr_code_path = format!("qrcodes/order_{}_qr.png", new_order.id);

        match PromptPayUtils::generate_qr(
            phone_number,
            total_amount.to_f64().unwrap(),
            &qr_code_path,
        ) {
            Ok(_) => {
                println!("QR Code saved to: {}", qr_code_path);
            }
            Err(e) => {
                eprintln!("Failed to generate QR Code: {}", e);
            }
        }

        Ok(order_model)
    }

    /// Retrieves an order with its associated items
    pub async fn get_order_with_items(
        db: &DatabaseConnection,
        order_id: i32,
    ) -> Result<(OrderModel, Vec<OrderItemModel>), DbErr> {
        let order = order::Entity::find_by_id(order_id).one(db).await?;
        let items = order_item::Entity::find()
            .filter(order_item::Column::OrderId.eq(order_id))
            .all(db)
            .await?;

        match order {
            Some(order) => {
                let order_model = OrderModel::new(
                    order.id,
                    order.user_id,
                    order.total_amount,
                    order.payment_status,
                );
                let order_item_models = items
                    .into_iter()
                    .map(|item| {
                        OrderItemModel::new(
                            item.id,
                            item.order_id,
                            item.product_id,
                            item.quantity,
                            item.price,
                        )
                    })
                    .collect();

                Ok((order_model, order_item_models))
            }
            None => Err(DbErr::RecordNotFound(format!(
                "Order with ID {} not found",
                order_id
            ))),
        }
    }

    /// Updates the payment status of an order
    pub async fn update_payment_status(
        db: &DatabaseConnection,
        order_id: i32,
        request: UpdatePaymentStatusRequest,
    ) -> Result<OrderModel, DbErr> {
        // Validate the input
        request
            .validate()
            .map_err(|err| DbErr::Custom(format!("Validation failed: {}", err)))?;

        let order = order::Entity::find_by_id(order_id)
            .one(db)
            .await?
            .ok_or_else(|| {
                DbErr::RecordNotFound(format!("Order with ID {} not found", order_id))
            })?;

        let mut active_model: order::ActiveModel = order.into();
        active_model.payment_status = Set(request.new_status);
        active_model.updated_at = Set(chrono::Utc::now().into());

        let updated_order = active_model.update(db).await?;

        let order_model = OrderModel::new(
            updated_order.id,
            updated_order.user_id,
            updated_order.total_amount,
            updated_order.payment_status,
        );

        Ok(order_model)
    }

    /// Deletes an order and its items
    pub async fn delete_order(db: &DatabaseConnection, order_id: i32) -> Result<(), DbErr> {
        let transaction = db.begin().await?;

        // Delete order items
        order_item::Entity::delete_many()
            .filter(order_item::Column::OrderId.eq(order_id))
            .exec(&transaction)
            .await?;

        // Delete the order
        order::Entity::delete_by_id(order_id)
            .exec(&transaction)
            .await?;

        transaction.commit().await?;
        Ok(())
    }

    // Helper function to get all carts for the user
    pub async fn get_all_carts_for_user(
        db: &DatabaseConnection,
        user_id: i32,
    ) -> Result<Vec<CartResponse>, ApiError> {
        let carts = cart::Entity::find()
            .filter(cart::Column::UserId.eq(user_id)) // Fetch all carts for the user
            .all(db)
            .await?;

        // Convert the result into the desired response format
        Ok(carts.into_iter().map(CartResponse::from).collect())
    }
    // Function to fetch all items from a specific cart
    pub async fn get_cart_items_for_user(
        db: &DatabaseConnection,
        cart_id: i32,
    ) -> Result<Vec<CartItemResponse>, ApiError> {
        let cart_items = cart_item::Entity::find()
            .filter(cart_item::Column::CartId.eq(cart_id)) // Fetch all items in a specific cart
            .all(db)
            .await?;

        // Convert the result into the desired response format
        Ok(cart_items.into_iter().map(CartItemResponse::from).collect())
    }

    // Service function to clear all carts for a user
    pub async fn clear_all_carts(db: &DatabaseConnection, user_id: i32) -> Result<(), ApiError> {
        use crate::entities::{cart, cart_item};
        use sea_orm::{ColumnTrait, EntityTrait, QueryFilter, TransactionTrait};

        // Start a transaction to ensure atomicity
        let txn = db.begin().await?;

        // Fetch all cart IDs for the user
        let cart_ids: Vec<i32> = cart::Entity::find()
            .filter(cart::Column::UserId.eq(user_id))
            .all(&txn)
            .await?
            .into_iter()
            .map(|cart| cart.id)
            .collect();

        if cart_ids.is_empty() {
            return Ok(()); // No carts to clear
        }

        // Delete all cart items for the fetched cart IDs
        cart_item::Entity::delete_many()
            .filter(cart_item::Column::CartId.is_in(cart_ids.clone()))
            .exec(&txn)
            .await?;

        // Delete all carts for the user
        cart::Entity::delete_many()
            .filter(cart::Column::UserId.eq(user_id))
            .exec(&txn)
            .await?;

        // Commit the transaction
        txn.commit().await?;

        Ok(())
    }
    /// Fetch price for a given product ID
    pub async fn get_product_price(
        db: &DatabaseConnection,
        product_id: i32,
    ) -> Result<Decimal, ApiError> {
        use crate::entities::product;

        // Fetch the product by its ID
        let product = product::Entity::find_by_id(product_id)
            .one(db)
            .await?
            .ok_or_else(|| {
                ApiError::NotFound(format!("Product with ID {} not found", product_id))
            })?;

        // Return the product's price
        Ok(product.price)
    }
}
