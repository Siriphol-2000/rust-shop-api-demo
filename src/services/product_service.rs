use crate::entities::product;
use crate::models::product::{ProductRequest, ProductResponse};
use crate::utils::actix_error::ApiError;
use sea_orm::entity::ModelTrait;
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, Set};

use chrono::Utc;

pub async fn create_product(
    db: &DatabaseConnection,
    request: ProductRequest,
) -> Result<ProductResponse, ApiError> {
    // Get the current timestamp
    let now_utc = Utc::now();
    let now_fixed: chrono::DateTime<chrono::FixedOffset> = now_utc.into(); // Convert to FixedOffset

    // Create a new product record with timestamps
    let new_product = product::ActiveModel {
        name: Set(request.name),
        description: Set(request.description),
        price: Set(request.price),
        created_at: Set(now_fixed), // Set the created_at timestamp
        updated_at: Set(now_fixed), // Set the updated_at timestamp
        ..Default::default()
    };

    // Insert the product into the database
    let result = new_product
        .insert(db)
        .await?;

    // Return the response with product details
    Ok(ProductResponse {
        id: result.id,
        name: result.name,
        description: result.description,
        price: result.price,
    })
}

/// Fetches a product by its ID
pub async fn get_product_by_id(
    db: &DatabaseConnection,
    product_id: i32,
) -> Result<ProductResponse, ApiError> {
    // Fetch the product by ID
let product=product::Entity::find_by_id(product_id).one(db).await?;
match product {
    Some(product)=>Ok(ProductResponse{
        id: product.id,
        name: product.name,
        description: product.description,
        price: product.price,
    }),
    None => Err(ApiError::NotFound(format!("Product with ID {} not found", product_id))),

}
    
} 

pub async fn get_all_products(db: &DatabaseConnection) -> Result<Vec<ProductResponse>, ApiError> {
    // Fetch all products
    let products = product::Entity::find()
        .all(db)
        .await?; // Assuming the error is a string

    // Convert the Vec<product::Model> to Vec<ProductResponse>
    let product_responses = products
        .iter()
        .map(|product| ProductResponse {
            id: product.id,
            name: product.name.clone(),
            description: product.description.clone(),
            price: product.price,
        })
        .collect();

    Ok(product_responses)
}

/// Updates an existing product
pub async fn update_product(
    db: &DatabaseConnection,
    product_id: i32,
    request: ProductRequest,
) -> Result<ProductResponse, ApiError> {
    let now_utc = Utc::now();
    let now_fixed: chrono::DateTime<chrono::FixedOffset> = now_utc.into(); // Convert to FixedOffset
                                                                           // Fetch the existing product by ID
    let product =  product::Entity::find_by_id(product_id).one(db).await?;
      match product {
    Some(product) => Ok(ProductResponse{
        id: product.id,
        name: product.name,
        description: product.description,
        price: product.price,
    }),
    None => Err(ApiError::NotFound(format!("Product with ID {} not found", product_id))),
    };

    // Convert the fetched model to ActiveModel for updates
    let updated_product = product::ActiveModel {
        id: Set(product_id), // Setting the ID so that we can update the record
        name: Set(request.name),
        description: Set(request.description),
        price: Set(request.price),
        updated_at: Set(now_fixed),
        ..Default::default() // Keep the rest of the fields unchanged
    };

    // Update the product in the database
    let updated_product = updated_product
        .update(db)
        .await?;

    // Return the updated product details
    Ok(ProductResponse {
        id: updated_product.id,
        name: updated_product.name,
        description: updated_product.description,
        price: updated_product.price,
    })
}

/// Deletes a product by its ID
/// Deletes a product by its ID
pub async fn delete_product(
    db: &DatabaseConnection,
    product_id: i32,
) -> Result<(), ApiError> {
    // Find the product by its ID
    let product = product::Entity::find_by_id(product_id)
        .one(db)
        .await?
        .ok_or_else(|| ApiError::NotFound(format!("Product with ID {} not found", product_id)))?;

    // Delete the product from the database
    product
        .delete(db)
        .await?;

    Ok(())
}

