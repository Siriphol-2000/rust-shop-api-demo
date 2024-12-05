use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, Set};
use crate::entities::prelude::*;
use crate::entities::user;
use crate::models::user::{UserRegisterRequest, UserResponse};
use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHash, PasswordHasher, PasswordVerifier, SaltString
    },
    Argon2
};

pub async fn register_user(
    db: &DatabaseConnection,
    request: UserRegisterRequest,
) -> Result<UserResponse, String> {
    // Generate a random salt for password hashing
    let salt = SaltString::generate(&mut OsRng);

    // Argon2 with default params (Argon2id v19)
    let argon2 = Argon2::default();

    // Hash the password using Argon2
    let password_hash = argon2
        .hash_password(request.password.as_bytes(), &salt)
        .map_err(|_| "Failed to hash password")?
        .to_string(); // Get the password hash as a string

    // Create a new user record
    let new_user = user::ActiveModel {
        email: Set(request.email.clone()),
        password_hash: Set(password_hash),
        ..Default::default()
    };

    // Insert the user into the database
    let result = new_user.insert(db).await.map_err(|_| "Failed to save user")?;

    // Return the response with user details
    Ok(UserResponse {
        id: result.id,
        email: result.email,
    })
}
