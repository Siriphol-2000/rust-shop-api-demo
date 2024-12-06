use crate::entities::prelude::*;
use crate::entities::product;
use crate::models::product::{ProductRequest, ProductResponse};
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, Set};
use sea_orm::QueryFilter;
use sea_orm::DbErr;
use sea_orm::entity::ModelTrait; 

/// Registers a new product
pub async fn create_product(
    db: &DatabaseConnection,
    request: ProductRequest,
) -> Result<ProductResponse, String> {
    // Create a new product record
    let new_product = product::ActiveModel {
        name: Set(request.name),
        description: Set(request.description),
        price: Set(request.price),
        ..Default::default()
    };

    // Insert the product into the database
    let result = new_product
        .insert(db)
        .await
        .map_err(|_| "Failed to save product")?;

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
) -> Result<ProductResponse, String> {
    // Fetch the product by ID
    match product::Entity::find_by_id(product_id).one(db).await {
        Ok(Some(product)) => Ok(ProductResponse {
            id: product.id,
            name: product.name,
            description: product.description,
            price: product.price,
        }),
        Ok(None) => Err(format!("Product with ID {} not found", product_id)),
        Err(err) => Err(format!("Failed to fetch product: {}", err)),
    }
}

pub async fn get_all_products(db: &DatabaseConnection) -> Result<Vec<ProductResponse>, String> {
    // Fetch all products
    let products = product::Entity::find()
        .all(db)
        .await
        .map_err(|_| "Failed to fetch products")?;// Assuming the error is a string

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
) -> Result<ProductResponse, String> {
    // Fetch the existing product by ID
    let product = match product::Entity::find_by_id(product_id)
        .one(db)
        .await
    {
        Ok(Some(product)) => product,
        Ok(None) => return Err(format!("Product with ID {} not found", product_id)),
        Err(err) => return Err(format!("Failed to fetch product: {}", err)),
    };

    // Convert the fetched model to ActiveModel for updates
    let mut updated_product = product::ActiveModel {
        id: Set(product.id), // Setting the ID so that we can update the record
        name: Set(request.name),
        description: Set(request.description),
        price: Set(request.price),
        ..Default::default() // Keep the rest of the fields unchanged
    };

    // Update the product in the database
    let updated_product = updated_product
        .update(db)
        .await
        .map_err(|_| "Failed to update product")?;

    // Return the updated product details
    Ok(ProductResponse {
        id: updated_product.id,
        name: updated_product.name,
        description: updated_product.description,
        price: updated_product.price,
    })
}


/// Deletes a product by its ID
pub async fn delete_product(
    db: &DatabaseConnection,
    product_id: i32,
) -> Result<(), String> {
    // Find the product by its ID
    let product: Option<product::Model> = product::Entity::find_by_id(product_id)
        .one(db)
        .await
        .map_err(|_| "Failed to find product")?;

    // Check if the product exists
    let product = product.ok_or("Product not found")?;

    // Delete the product
    let delete_result = product.delete(db).await.map_err(|_| "Failed to delete product")?;

    // Ensure that one row was affected
    if delete_result.rows_affected == 0 {
        return Err("No rows were affected, product might not exist".to_string());
    }

    Ok(())
}

