use serde::{Deserialize, Serialize};
use validator::Validate;

/// This struct is used to handle user registration requests
#[derive(Debug, Deserialize, Validate)]
pub struct UserRegisterRequest {
    #[validate(email(message = "Invalid email format"))]
    pub email: String,

    #[validate(length(min = 6, message = "Password must be at least 6 characters long"))]
    pub password: String,
}

/// This struct is used to return user details in the API responses
#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub id: i32,
    pub email: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UserLoginRequest {
    #[validate(email(message = "Invalid email format"))]
    pub email: String,

    #[validate(length(min = 6, message = "Password must be at least 6 characters long"))]
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct UserLoginResponse {
    pub id: i32,
    pub email: String,
    pub token: String,
}
