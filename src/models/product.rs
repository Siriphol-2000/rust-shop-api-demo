use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError};

/// This struct is used to handle the creation or update of a product.
#[derive(Debug, Deserialize, Validate)]
pub struct ProductRequest {
    #[validate(length(min = 1, message = "Product name cannot be empty."))]
    pub name: String,

    #[validate(length(max = 100, message = "Description cannot exceed 100 characters."))]
    pub description: Option<String>,

    #[validate(custom(function = "validate_decimal_range"))]
    pub price: Decimal,
}

/// This struct is used to return product details in API responses.
#[derive(Debug, Serialize)]
pub struct ProductResponse {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub price: Decimal,
}

impl From<crate::entities::product::Model> for ProductResponse {
    fn from(model: crate::entities::product::Model) -> Self {
        ProductResponse {
            id: model.id,
            name: model.name,
            description: model.description,
            price: model.price,
        }
    }
}

/// Custom validator for the `Decimal` type to check that the value is not less than zero.
fn validate_decimal_range(value: &Decimal) -> Result<(), ValidationError> {
    if *value < Decimal::new(0, 0) {
        let mut error = ValidationError::new("price_validation");
        error.message = Some("Price must be zero or a positive value.".into());
        return Err(error);
    }
    Ok(())
}