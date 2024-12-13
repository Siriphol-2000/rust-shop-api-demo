use crate::entities::user;
use crate::models::user::{UserLoginResponse, UserRegisterRequest, UserResponse};
use crate::utils::actix_error::ApiError;
use crate::utils::jwt::generate_jwt;
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use sea_orm::QueryFilter;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, Set};

pub async fn register_user(
    db: &DatabaseConnection,
    request: UserRegisterRequest,
) -> Result<UserResponse, ApiError> {
    // Generate a random salt for password hashing
    let salt = SaltString::generate(&mut OsRng);

    // Argon2 with default params (Argon2id v19)
    let argon2 = Argon2::default();

    // Hash the password using Argon2
    let password_hash = argon2
    .hash_password(request.password.as_bytes(), &salt)
    .map_err(|_| ApiError::InternalServerError("Failed to hash password".to_string()))? // Use String for the error
    .to_string(); // Convert PasswordHash to String


    // Create a new user record
    let new_user = user::ActiveModel {
        email: Set(request.email.clone()),
        password_hash: Set(password_hash),
        ..Default::default()
    };

    // Insert the user into the database
    let result = new_user
        .insert(db)
        .await?;

    // Return the response with user details
    Ok(UserResponse {
        id: result.id,
        email: result.email,
    })
}
pub async fn get_user_by_id(db: &DatabaseConnection, user_id: i32) -> Result<UserResponse, ApiError> {
    // Fetch the user by ID
    let user = user::Entity::find_by_id(user_id).one(db).await?;
    
    match user {
        Some(user)=>Ok(UserResponse{
            id: user.id,
            email: user.email,
        }),
        None => Err(ApiError::NotFound(format!("User with ID {} not found", user_id))),
    }
    // match user::Entity::find_by_id(user_id).one(db).await {
    //     Ok(Some(user)) => Ok(UserResponse {
    //         id: user.id,
    //         email: user.email,
    //     }),
    //     Ok(None) => Err(format!("User with ID {} not found", user_id)),
    //     Err(err) => Err(format!("Failed to fetch user: {}", err)),
    // }
}

pub async fn authenticate_user(
    db: &DatabaseConnection,
    email: &str,
    password: &str,
) -> Result<UserLoginResponse, ApiError> {
  // Fetch user by email and propagate any database errors with `?`
  let user = user::Entity::find()
  .filter(user::Column::Email.eq(email))
  .one(db)
  .await?;

// Check if the user was found
let user = user.ok_or(ApiError::NotFound(format!("User with email {} not found", email)))?;

// Verify password
let parsed_hash = PasswordHash::new(&user.password_hash)
  .map_err(|_| ApiError::AuthenticationError("Invalid password hash".to_string()))?;

if Argon2::default()
  .verify_password(password.as_bytes(), &parsed_hash)
  .is_err()
{
  return Err(ApiError::AuthenticationError("Invalid credentials".to_string()));
}

// Generate JWT token for the user
let token = generate_jwt(user.id, user.email.clone())
  .map_err(|_| ApiError::InternalServerError("Failed to generate JWT".to_string()))?;

// Return the response with the token
Ok(UserLoginResponse {
  id: user.id,
  email: user.email,
  token,
})
}
